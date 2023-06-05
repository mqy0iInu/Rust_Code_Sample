use std::time::Duration;

// ********************************
// ============[SDL2]=============
use sdl2::event::Event;
 // For PC Display
use sdl2::rect::Rect;
use sdl2::pixels::Color;
 // For PC Keyboard
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
// ********************************
const KEY_MOVE_VAL      :u32  = 10;
const WINDOW_WIDTH      :u32 = 800;
const WINDOW_HIGHT      :u32 = 600;
const CHAR_MOVE_WIDTH   :u32 = 350;
const CHAR_MOVE_HIGHT   :u32 = 250;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Rust-SDL2", WINDOW_WIDTH, WINDOW_HIGHT)
        .position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let (mut x, mut y) = (CHAR_MOVE_WIDTH, CHAR_MOVE_HIGHT);

    'running: loop {
        let state = event_pump.keyboard_state();
        if state.is_scancode_pressed(Scancode::Up) {
            y -= KEY_MOVE_VAL;
        }
        if state.is_scancode_pressed(Scancode::Down) {
            y += KEY_MOVE_VAL;
        }
        if state.is_scancode_pressed(Scancode::Left) {
            x -= KEY_MOVE_VAL;
        }
        if state.is_scancode_pressed(Scancode::Right) {
            x += KEY_MOVE_VAL;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(x.try_into().unwrap(), y.try_into().unwrap(), 100, 100)).unwrap();

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}