use crate::common::{map, Rect, Vec2};
use crate::gfx::draw_pixel;
use crate::player::Player;
use crate::wasm4;
use heapless::HistoryBuffer;

const ROTATE_THE_WORLD: bool = true;
pub struct World {
    view: Rect,
    player: Player,
    planets: HistoryBuffer<Planet, 255>,

    seconds_passed: u32,
}

impl World {
    pub const fn new() -> Self {
        Self {
            view: Rect::new(Vec2::new(-80.0, -80.0)),
            player: Player::new(Vec2::new(0.0, 0.0)),
            planets: HistoryBuffer::new(),
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

        self.player.update();
        let player_pos = self.player.pos;
        self.change_view(player_pos);

        if self.seconds_passed < time as u32 {
            self.seconds_passed += 1;
        }

        if self.count_planets() < 20 {
            let possible_planet = Planet::new(
                Vec2::new(
                    self.view.center().x + (random.rand_float() - 0.5) * 300.,
                    self.view.center().y + (random.rand_float() - 0.5) * 300.,
                ),
                random.rand_float() * 30. + 5.,
            );
            if self.planets.iter().all(|planet| {
                planet.pos.distance(possible_planet.pos)
                    > ((planet.radius + possible_planet.radius) * 3.0)
                    && possible_planet.pos.distance(player_pos) > 114.0 + possible_planet.radius
            }) {
                self.planets.write(possible_planet);
            }
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

    pub fn draw(&self) {
        let view = &self.view;
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
}

impl Planet {
    pub const fn new(pos: Vec2, radius: f32) -> Self {
        Self { pos, radius }
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
                    0.0,   //self.color_center as f32,
                    255.0, //self.color_end as f32,
                );
                draw_pixel(u32::from(screen_x), u32::from(screen_y), color as u8);
            }
        }
    }
}
