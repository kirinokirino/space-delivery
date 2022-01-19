use crate::common::{map, Rect, Vec2};
use crate::gfx::draw_pixel;
use crate::player::Player;
use crate::wasm4;
use core::f32::consts::PI;
use heapless::HistoryBuffer;
pub struct World {
    view: Rect,
    player: Player,
    planets: HistoryBuffer<Planet, 255>,
    stars: HistoryBuffer<Star, 255>,
    seconds_passed: u32,
}

impl World {
    pub const fn new() -> Self {
        let stars = HistoryBuffer::new();
        Self {
            view: Rect::new(Vec2::new(-80.0, -80.0)),
            player: Player::new(Vec2::new(0.0, 0.0), true),
            planets: HistoryBuffer::new(),
            stars,
            seconds_passed: 0,
        }
    }

    pub fn handle_gamepad(&mut self, gamepad: u8) {
        self.player.handle_gamepad(gamepad);
    }

    pub fn mouse_click(&mut self, mouse: (i16, i16)) {
        let (x, y) = mouse;
        let pos = Vec2::new(
            self.view.top_left.x + f32::from(x),
            self.view.top_left.y + f32::from(y),
        );
    }

    pub fn update(&mut self, time: f64) {
        let mut random = oorandom::Rand32::new(unsafe { crate::FRAME_COUNT.into() });

        let player_pos = self.player.pos;
        for planet in self.planets.as_slice() {
            let delta = planet.pos - player_pos;
            let distance = delta.magnitude();
            if distance < 114.0 * 3.0 && distance > planet.radius {
                let force = planet.radius * PI / (distance * distance);
                self.player.apply_force(delta.normalized() * force);
            }
        }
        self.player.update();
        self.change_view(player_pos);

        if self.seconds_passed < time as u32 {
            self.seconds_passed += 1;
        }

        if self.count_planets() < 20 {
            let planet_size: u8 =
                unsafe { (random.rand_u32() % 200).try_into().unwrap_unchecked() };
            let possible_planet = Planet::new(
                Vec2::new(
                    self.view.center().x + (random.rand_float() - 0.5) * 300.,
                    self.view.center().y + (random.rand_float() - 0.5) * 300.,
                ),
                random.rand_float() * 30. + 5.,
                planet_size + 55u8,
            );
            #[allow(clippy::collapsible_if)]
            if possible_planet.pos.distance(player_pos) > 114.0 + possible_planet.radius {
                if self.planets.iter().all(|planet| {
                    planet.pos.distance(possible_planet.pos)
                        > ((planet.radius + possible_planet.radius) * 3.0)
                }) {
                    self.planets.write(possible_planet);
                }
            }
        }

        if self.count_stars() < 35 {
            self.gen_star(random);
        }
    }

    fn gen_star(&mut self, mut random: oorandom::Rand32) {
        let star = Star::new(
            Vec2::new(
                self.view.center_mul(3.0).x + (random.rand_float() - 0.5) * 1000.,
                self.view.center_mul(3.0).y + (random.rand_float() - 0.5) * 1000.,
            ),
            unsafe { (random.rand_u32() % 200).try_into().unwrap_unchecked() },
            random.rand_float() * 2.0 + 0.5,
        );
        let distance = star.pos.distance(self.view.center_mul(3.0));
        if distance > 114.0 * 3.0 && distance < 114.0 * 5.0 {
            self.stars.write(star);
        }
    }

    pub fn change_view(&mut self, pos: Vec2) {
        let pos = Vec2::new(pos.x - 80.0, pos.y - 80.0);
        self.view.change_pos(pos);
    }

    pub fn count_planets(&self) -> u8 {
        self.planets
            .iter()
            .filter(|planet| planet.pos.distance(self.view.center()) < 114.0 * 5.0)
            .count() as u8
    }

    pub fn count_stars(&self) -> u8 {
        self.stars
            .iter()
            .filter(|star| star.pos.distance(self.view.center_mul(3.0)) < 114.0 * 5.0)
            .count() as u8
    }

    pub fn draw(&self) {
        let view = &self.view;
        self.stars
            .as_slice()
            .iter()
            .filter(|star| star.pos.distance(self.view.center_mul(3.0)) < 114.0 * 3.0)
            .for_each(|star| star.draw(view));

        self.planets
            .as_slice()
            .iter()
            .filter(|planet| planet.pos.distance(self.view.center()) < 114.0 + planet.radius)
            .for_each(|planet| planet.draw(view));

        let planets = self.count_planets();
        wasm4::line(10, 130, 10, 130 - i32::from(planets));

        self.player.draw(view);
    }
}

struct Planet {
    pos: Vec2,
    radius: f32,
    color: u8,
}

impl Planet {
    pub const fn new(pos: Vec2, radius: f32, color: u8) -> Self {
        Self { pos, radius, color }
    }

    pub fn draw(&self, view: &Rect) {
        let left = view.top_left.x;
        let top = view.top_left.y;

        for screen_y in 0..160u8 {
            for screen_x in 0..160u8 {
                let screen_point = Vec2::new(f32::from(screen_x) + left, f32::from(screen_y) + top);
                let distance = self.pos.distance(screen_point);
                if distance > self.radius {
                    continue;
                }
                let color = map(
                    distance,
                    0.0,
                    self.radius,
                    0.0,                   //self.color_center as f32,
                    f32::from(self.color), //self.color_end as f32,
                );
                draw_pixel(screen_x, screen_y, color as u8);
            }
        }
    }
}

struct Star {
    pos: Vec2,
    color: u8,
    size: f32,
}

impl Star {
    pub const fn new(pos: Vec2, color: u8, size: f32) -> Self {
        Self { pos, color, size }
    }

    pub fn draw(&self, view: &Rect) {
        let left = view.top_left.x / 3.0;
        let top = view.top_left.y / 3.0;

        for screen_y in 0..160u8 {
            for screen_x in 0..160u8 {
                let screen_point = Vec2::new(f32::from(screen_x) + left, f32::from(screen_y) + top);
                let distance = (self.pos * (1. / 3.)).distance(screen_point);
                if distance > self.size {
                    continue;
                }
                draw_pixel(screen_x, screen_y, self.color);
            }
        }
    }
}
