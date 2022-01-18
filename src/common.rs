use core::f32::consts::{FRAC_1_PI, PI};
use core::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance(self, other: Self) -> f32 {
        let delta_x = self.x - other.x;
        let delta_y = self.y - other.y;

        sqrt(delta_x * delta_x + delta_y * delta_y)
    }

    pub fn rotated(&self, rad: f32) -> Self {
        Self {
            x: self.x * cos(rad) - self.y * sin(rad),
            y: self.x * sin(rad) + self.y * cos(rad),
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
pub struct Rect {
    pub top_left: Vec2,
    width: u16,
    height: u16,
}

impl Rect {
    pub const fn new(top_left: Vec2) -> Self {
        let width = 160;
        let height = 160;
        Self {
            top_left,
            width,
            height,
        }
    }

    pub fn change_pos(&mut self, top_left: Vec2) {
        self.top_left = top_left;
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.top_left.x + (f32::from(self.width) / 2.),
            self.top_left.y + (f32::from(self.height) / 2.),
        )
    }
}

pub fn map(value: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    (value - start1) / (stop1 - start1) * (stop2 - start2) + start2
}

pub fn norm(value: f32, start: f32, stop: f32) -> f32 {
    map(value, start, stop, 0.0, 1.0)
}

pub fn sqrt(value: f32) -> f32 {
    if value >= 0.0 {
        f32::from_bits((value.to_bits() + 0x3f80_0000) >> 1)
    } else {
        f32::NAN
    }
}

pub fn cos(value: f32) -> f32 {
    let mut x = value;
    x *= FRAC_1_PI / 2.0;
    x -= 0.25 + floor(x + 0.25);
    x *= 16.0 * (abs(x) - 0.5);
    x += 0.225 * x * (abs(x) - 1.0);
    x
}

pub fn sin(value: f32) -> f32 {
    cos(value - PI / 2.0)
}

pub fn floor(value: f32) -> f32 {
    let mut res = (value as i32) as f32;

    if value < res {
        res -= 1.0;
    }
    res
}

pub fn abs(value: f32) -> f32 {
    if value < 0.0 {
        -value
    } else {
        value
    }
}
