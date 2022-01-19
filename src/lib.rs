#![no_std]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::unwrap_in_result,
    clippy::unneeded_field_pattern,
    clippy::string_to_string,
    clippy::string_slice,
    clippy::string_add,
    clippy::str_to_string,
    clippy::same_name_method,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::rc_mutex,
    clippy::rc_buffer,
    clippy::pattern_type_mismatch,
    clippy::multiple_inherent_impl,
    clippy::missing_enforced_import_renames,
    clippy::lossy_float_literal,
    clippy::let_underscore_must_use,
    clippy::integer_division,
    clippy::inline_asm_x86_att_syntax,
    clippy::indexing_slicing,
    clippy::if_then_some_else_none,
    clippy::get_unwrap,
    clippy::fn_to_numeric_cast,
    clippy::float_cmp_const,
    clippy::filetype_is_file,
    clippy::create_dir,
    clippy::clone_on_ref_ptr,
    clippy::as_conversions,
    clippy::verbose_file_reads
)]
#![allow(clippy::suboptimal_flops)]
mod common;
mod gfx;
mod player;
mod wasm4;
mod world;

#[cfg(feature = "buddy-alloc")]
mod alloc;

use common::Vec2;
use world::World;

const CURSOR_SIZE: u8 = 4;
const MOUSE_CURSOR: [u8; 2] = [0b1110_1000, 0b1010_0001];

static mut FRAME_COUNT: u32 = 0;
static mut WORLD: World = World::new();

#[no_mangle]
fn start() {
    let mut random = unsafe { oorandom::Rand32::new(FRAME_COUNT.into()) };
    let radius = random.rand_float() * 10.0 + 5.0;
    unsafe {
        *wasm4::PALETTE = [0x002d_162c, 0x0041_2752, 0x0068_3a68, 0x0097_75a6];
        *wasm4::DRAW_COLORS = 0x4321;
    }
}

#[no_mangle]
fn update() {
    let time = unsafe { f64::from(FRAME_COUNT) / 60. };
    let mut random = unsafe { oorandom::Rand32::new(FRAME_COUNT.into()) };
    let gamepad = unsafe { *wasm4::GAMEPAD1 };
    unsafe {
        WORLD.handle_gamepad(gamepad);
    }

    let mouse = unsafe { (*wasm4::MOUSE_X, *wasm4::MOUSE_Y) };
    let mouse_pressed = unsafe { *wasm4::MOUSE_BUTTONS & wasm4::MOUSE_LEFT };

    if mouse_pressed != 0 {
        unsafe {
            WORLD.mouse_click(mouse);
        }
    }

    unsafe {
        WORLD.update(time);
    }

    unsafe {
        WORLD.draw();
    }

    unsafe {
        FRAME_COUNT += 1;
    }

    {
        unsafe {
            *wasm4::DRAW_COLORS = 3;
        }
        let x: i32 = mouse.0.into();
        let y: i32 = mouse.1.into();
        wasm4::blit(
            &MOUSE_CURSOR,
            x,
            y,
            CURSOR_SIZE.into(),
            CURSOR_SIZE.into(),
            wasm4::BLIT_1BPP,
        );
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        wasm4::trace(s);
    } else {
        wasm4::trace("panic occurred");
    }
    loop {}
}
