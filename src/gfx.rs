#![allow(clippy::integer_division, clippy::as_conversions)]
use crate::wasm4;

const COLORS: u8 = 4;
const MAX_BRIGHTNESS: u8 = 255;
const DISPERSION_MATRIX_SIZE: u8 = 9;
const DISPERSED: [u8; DISPERSION_MATRIX_SIZE as usize] = [1, 7, 4, 5, 8, 3, 6, 2, 9];

pub fn draw_pixel(x: u32, y: u32, brightness: u8) {
    if x > 160 || y > 160 {
        return;
    }
    // Additional color options.
    let step = MAX_BRIGHTNESS / (COLORS - 1);

    let color_base = brightness / step;
    // Get the value from the matrix.
    let idx = ((x - y * 3) % u32::from(DISPERSION_MATRIX_SIZE)) as usize;
    let threshhold = unsafe { DISPERSED.get_unchecked(idx) * (step / DISPERSION_MATRIX_SIZE) };
    let diff = brightness % step;
    let add_color = if diff >= threshhold { 1 } else { 0 };
    let draw_color = color_base + add_color;
    let draw_color = 1 + u16::from(draw_color);

    unsafe {
        *wasm4::DRAW_COLORS = draw_color;
    }
    screen_pixel(x, y);
}

fn screen_pixel(x: u32, y: u32) {
    // The byte index into the framebuffer that contains (x, y)
    let idx = (y as usize * 160 + x as usize) >> 2;

    // Calculate the bits within the byte that corresponds to our position
    let shift = (x as u8 & 0b11) << 1;
    let mask = 0b11 << shift;

    unsafe {
        let palette_color: u8 = (*wasm4::DRAW_COLORS & 0xf) as u8;
        if palette_color == 0 {
            // Transparent
            return;
        }
        let color = (palette_color - 1) & 0b11;

        let framebuffer = wasm4::FRAMEBUFFER.as_mut().expect("fb ref");

        framebuffer[idx] = (color << shift) | (framebuffer[idx] & !mask);
    }
}
