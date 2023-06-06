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
const CHAR_WIDTH        :u32 = 350;
const CHAR_HIGHT        :u32 = 250;

const SAFE_ADD :u8 = 0;
const SAFE_SUB :u8 = 1;
const SAFE_X :u8 = 0;
const SAFE_Y :u8 = 1;

pub fn safe_add_sub(xy :u32, val :u32, xy_type :u8, mode :u8,) -> u32
{
    if mode != SAFE_SUB {
        if xy_type != SAFE_Y
        {
            if xy >= (WINDOW_WIDTH - 100) {
                WINDOW_WIDTH - 100
            }else {
                xy + val
            }
        }else {
            if xy >= (WINDOW_HIGHT - 100) {
                WINDOW_HIGHT - 100
            }else {
                xy + val
            }
        }
    }else {
        if xy > val {
            xy - val
        }else {
            0
        }
    }
}

pub fn main()
{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Rust_SDL2", WINDOW_WIDTH, WINDOW_HIGHT)
        .position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let (mut x, mut y) = (CHAR_WIDTH, CHAR_HIGHT);

    'lab_loop: loop
    {
        let state = event_pump.keyboard_state();
        if state.is_scancode_pressed(Scancode::Up) {
            y = safe_add_sub(y, KEY_MOVE_VAL, SAFE_Y, SAFE_SUB);
        }
        if state.is_scancode_pressed(Scancode::Down) {
            y = safe_add_sub(y, KEY_MOVE_VAL, SAFE_Y, SAFE_ADD);
        }
        if state.is_scancode_pressed(Scancode::Left) {
            x = safe_add_sub(x, KEY_MOVE_VAL, SAFE_X, SAFE_SUB);
        }
        if state.is_scancode_pressed(Scancode::Right) {
            x = safe_add_sub(x, KEY_MOVE_VAL, SAFE_X, SAFE_ADD);
        }

        println!("X: {}, y: {}", x, y);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {break 'lab_loop},
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(x.try_into().unwrap(), y.try_into().unwrap(), 100, 100)).unwrap();

        canvas.present();

        std::thread::sleep(Duration::new(0, 10_0000_0000u32 / 60));
    }
}