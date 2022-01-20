use crate::common::{map, Rect, Vec2};
use crate::gfx::draw_pixel;

pub const MAX_LIFETIME: u16 = 600;
pub struct PhysicsObject {
    pub pos: Vec2,
    pub vel: Vec2,
    acc: Vec2,
    lifetime: Option<u16>,
}

impl PhysicsObject {
    pub const fn new(pos: Vec2, lifetime: Option<u16>) -> Self {
        Self {
            pos,
            vel: Vec2::new(0.0, 0.0),
            acc: Vec2::new(0.0, 0.0),
            lifetime,
        }
    }

    pub fn update(&mut self) {
        self.vel += self.acc;
        //self.vel = self.vel * 0.99;
        self.pos += self.vel;
        self.acc = Vec2::new(0.0, 0.0);
        if let Some(lifetime) = self.lifetime {
            match lifetime.checked_sub(1) {
                Some(lifetime) => self.lifetime = Some(lifetime),
                None => self.lifetime = Some(0),
            }
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acc += force;
    }

    pub fn debug_draw(&self, view: &Rect) {
        let mut color = 255u8;
        if let Some(lifetime) = self.lifetime {
            color = map(
                f32::from(lifetime),
                0.0,
                f32::from(MAX_LIFETIME),
                0.0,
                255.0,
            ) as u8;
        };
        if color != 0 {
            let left = view.top_left.x;
            let top = view.top_left.y;

            for screen_y in 0..160u8 {
                for screen_x in 0..160u8 {
                    let screen_point =
                        Vec2::new(f32::from(screen_x) + left, f32::from(screen_y) + top);
                    let distance = (self.pos).distance(screen_point);
                    if distance > 1.0 {
                        continue;
                    }

                    draw_pixel(screen_x, screen_y, color);
                }
            }
        }
    }
}
