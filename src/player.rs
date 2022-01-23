use crate::common::{abs, clamp, Rect, Vec2};
use crate::particle::{PhysicsObject, MAX_LIFETIME};
use crate::wasm4;

use core::f32::consts::PI;

const POWER: f32 = 0.02;
const MAX_SPEED: f32 = 2.0;

pub struct Player {
    pub physics: PhysicsObject,
    target: Option<Vec2>,
}

impl Player {
    pub const fn new(pos: Vec2) -> Self {
        Self {
            physics: PhysicsObject::new(pos, Some(MAX_LIFETIME)),
            target: None,
        }
    }

    pub fn handle_gamepad(&mut self, gamepad: u8) -> Option<Vec2> {
        if gamepad & wasm4::BUTTON_2 == 0 {
            let mut force = Vec2::new(0.0, 0.0);
            let speed = self.physics.vel;
            let delta_x = MAX_SPEED - abs(speed.x);
            let delta_y = MAX_SPEED - abs(speed.y);
            if gamepad & wasm4::BUTTON_UP != 0 {
                force = Vec2::new(0.0, clamp(-delta_y, -POWER, 0.0));
            } else if gamepad & wasm4::BUTTON_DOWN != 0 {
                force = Vec2::new(0.0, clamp(delta_y, 0.0, POWER));
            }
            if gamepad & wasm4::BUTTON_LEFT != 0 {
                force += Vec2::new(clamp(-delta_x, -POWER, 0.0), 0.0);
            } else if gamepad & wasm4::BUTTON_RIGHT != 0 {
                force += Vec2::new(clamp(delta_x, 0.0, POWER), 0.0);
            }
            if gamepad & wasm4::BUTTON_1 != 0 && self.physics.vel.magnitude() < 0.5 {
                force += Vec2::new(0.0, -MAX_SPEED);
            }
            self.apply_force(force);
            if force.magnitude() > 0.005 {
                self.target = None;
                return Some(force * -15.0);
            }
            return None;
        }
        self.target = Some(self.physics.pos);
        None
    }

    pub fn update(&mut self) {
        self.physics.update();
        if let Some(target) = self.target {
            self.apply_force(
                ((target - self.physics.pos) - self.physics.vel).normalized() * POWER * 2.5,
            );
        }
        if self.physics.vel.magnitude() > MAX_SPEED {
            self.physics.vel = self.physics.vel.normalized() * MAX_SPEED;
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.physics.apply_force(force);
    }

    pub fn collide(&mut self, to_collider: Vec2) {
        self.apply_force(self.physics.vel * -0.2);
    }

    #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
    pub fn draw2(&self, view: &Rect) {
        let size = 3;

        let left = view.top_left.x;
        let top = view.top_left.y;

        let x1 = self.physics.pos.x - left;
        let y1 = self.physics.pos.y - top;

        let start = Vec2::new(x1, y1 - 3.0);
        let front = self.physics.vel * 1.5 + start;
        let right = self.physics.vel.rotated(PI / 2.0) + start;
        let back = self.physics.vel * -1.5 + start;
        let left = self.physics.vel.rotated(-PI / 2.0) + start;
        unsafe {
            *wasm4::DRAW_COLORS = 4;
        }
        wasm4::line(left.x as i32, left.y as i32, front.x as i32, front.y as i32);
        wasm4::line(
            right.x as i32,
            right.y as i32,
            front.x as i32,
            front.y as i32,
        );
        wasm4::line(left.x as i32, left.y as i32, back.x as i32, back.y as i32);
        wasm4::line(right.x as i32, right.y as i32, back.x as i32, back.y as i32);
    }

    #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
    pub fn draw(&self, view: &Rect) {
        let size = 2;

        let left = view.top_left.x;
        let top = view.top_left.y;

        let x1 = self.physics.pos.x - left;
        let y1 = self.physics.pos.y - top;

        let start = Vec2::new(x1, y1 - 2.0);
        let end = self.physics.vel + start;
        unsafe {
            *wasm4::DRAW_COLORS = 4;
        }
        wasm4::line(
            x1 as i32 - size,
            y1 as i32 - size,
            x1 as i32 + size,
            y1 as i32 - size,
        );
        wasm4::line(
            start.x as i32 - size,
            start.y as i32,
            end.x as i32,
            end.y as i32,
        );
        wasm4::line(
            start.x as i32 + size,
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
