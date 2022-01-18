use crate::common::{Rect, Vec2};
use crate::wasm4;
use core::f32::consts::PI;

const ROT_POWER: f32 = 0.001;
const POWER: f32 = 0.02;

pub struct Player {
    pub pos: Vec2,
    vel: f32,
    acc: f32,
    rot: f32,
    rot_vel: f32,
    rot_acc: f32,
}

impl Player {
    pub const fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: 0.0,
            acc: 0.0,
            rot: PI,
            rot_vel: 0.0,
            rot_acc: 0.0,
        }
    }

    pub fn handle_gamepad(&mut self, gamepad: u8) {
        if gamepad & wasm4::BUTTON_LEFT != 0 {
            self.rot_acc = -ROT_POWER;
        } else if gamepad & wasm4::BUTTON_RIGHT != 0 {
            self.rot_acc = ROT_POWER;
        }
        if gamepad & wasm4::BUTTON_UP != 0 {
            self.acc = POWER;
        } else if gamepad & wasm4::BUTTON_DOWN != 0 {
            self.acc = -POWER;
        }
    }

    pub fn update(&mut self) {
        self.vel += self.acc;
        self.rot_vel += self.rot_acc;
        self.rot_vel *= 0.98;
        self.rot += self.rot_vel;
        self.change_pos(Vec2::new(0.0, self.vel).rotated(self.rot));
        self.acc = 0.0;
        self.rot_acc = 0.0;
    }

    pub fn change_pos(&mut self, delta: Vec2) {
        self.pos += delta;
    }

    pub fn draw(&self, view: &Rect) {
        let left = view.top_left.x;
        let top = view.top_left.y;

        let x1 = self.pos.x - left;
        let y1 = self.pos.y - top;

        let start = Vec2::new(x1, y1 - 4.0);
        let end = Vec2::new(0.0, self.vel).rotated(self.rot) * 4.0 + start;
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
