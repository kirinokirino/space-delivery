use crate::common::{Rect, Vec2};
use crate::gfx::draw_pixel;

pub struct PhysicsObject {
    pub pos: Vec2,
    pub vel: Vec2,
    acc: Vec2,
}

impl PhysicsObject {
    pub const fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: Vec2::new(0.0, 0.0),
            acc: Vec2::new(0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        self.vel += self.acc;
        //self.vel = self.vel * 0.99;
        self.pos += self.vel;
        self.acc = Vec2::new(0.0, 0.0);
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acc += force;
    }

    pub fn debug_draw(&self, view: &Rect) {
        let left = view.top_left.x;
        let top = view.top_left.y;

        for screen_y in 0..160u8 {
            for screen_x in 0..160u8 {
                let screen_point = Vec2::new(f32::from(screen_x) + left, f32::from(screen_y) + top);
                let distance = (self.pos).distance(screen_point);
                if distance > 1.0 {
                    continue;
                }
                draw_pixel(screen_x, screen_y, 255u8);
            }
        }
    }
}
