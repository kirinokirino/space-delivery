use crate::common::{abs, Rect, Vec2};
use crate::particle::PhysicsObject;
use crate::wasm4;

const POWER: f32 = 0.05;
const MAX_SPEED: f32 = 2.0;

pub struct Player {
    pub physics: PhysicsObject,
}

impl Player {
    pub const fn new(pos: Vec2, is_ai: bool) -> Self {
        Self {
            physics: PhysicsObject::new(pos),
        }
    }

    pub fn handle_gamepad(&mut self, gamepad: u8) {
        if gamepad & wasm4::BUTTON_1 != 0 {
            let speed = self.physics.vel;
            let delta_x = MAX_SPEED - abs(speed.x);
            let delta_y = MAX_SPEED - abs(speed.y);
            let mut force = Vec2::new(0.0, -POWER * delta_y);
            if gamepad & wasm4::BUTTON_LEFT != 0 {
                force += Vec2::new(-POWER * delta_x, 0.0);
            } else if gamepad & wasm4::BUTTON_RIGHT != 0 {
                force += Vec2::new(POWER * delta_x, 0.0);
            }
            self.apply_force(force);
        }
    }

    pub fn update(&mut self) {
        self.physics.update();

        if self.physics.vel.magnitude() > MAX_SPEED {
            self.physics.vel = self.physics.vel.normalized() * MAX_SPEED;
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.physics.apply_force(force);
    }

    pub fn collide(&mut self) {
        self.apply_force(self.physics.vel * -1.0);
    }

    #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
    pub fn draw(&self, view: &Rect) {
        let left = view.top_left.x;
        let top = view.top_left.y;

        let x1 = self.physics.pos.x - left;
        let y1 = self.physics.pos.y - top;

        let start = Vec2::new(x1, y1 - 4.0);
        let end = self.physics.vel + start;
        unsafe {
            *wasm4::DRAW_COLORS = 4;
        }
        wasm4::line(x1 as i32 - 4, y1 as i32 - 4, x1 as i32 + 4, y1 as i32 - 4);
        wasm4::line(
            start.x as i32 - 4,
            start.y as i32,
            end.x as i32,
            end.y as i32,
        );
        wasm4::line(
            start.x as i32 + 4,
            start.y as i32,
            end.x as i32,
            end.y as i32,
        );
        wasm4::line(
            start.x as i32,
            start.y as i32 + 2,
            end.x as i32,
            end.y as i32,
        );
    }
}
