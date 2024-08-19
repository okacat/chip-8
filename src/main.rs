mod chip8;

extern crate sdl2;

use chip8::{
    decode_instruction, execute_instruction, fetch_instruction, Chip8, SCREEN_HEIGHT, SCREEN_WIDTH,
};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const COLOR_OFF: Color = Color::RGB(0x10, 0x1D, 0x42);
const COLOR_ON: Color = Color::RGB(0xF7, 0x87, 0x64);
const DISP_SCALE: u32 = 8;

fn main() {
    println!("CHIP-8");
    println!("");
    println!("welcome to my CHIP-8 emulator :)");

    let rom = load_file("Tetris [Fran Dachille, 1991].ch8");
    let mut chip8 = Chip8::new();
    chip8.load_font();
    chip8.load_into_mem(&rom, 0x200);
    chip8.regs.pc = 0x200;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "chip-8",
            SCREEN_WIDTH as u32 * DISP_SCALE,
            SCREEN_HEIGHT as u32 * DISP_SCALE,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(COLOR_OFF);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = keycode_to_button(keycode) {
                        chip8.key_down[key] = true;
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = keycode_to_button(keycode) {
                        chip8.key_down[key] = false;
                    }
                }
                _ => {}
            }
        }

        // clear screen
        canvas.set_draw_color(COLOR_OFF);
        canvas.clear();

        // roughly 11 instructions per frame, going by folklore
        for _ in 0..11 {
            // do one step of chip-8
            let raw_instruction = fetch_instruction(&mut chip8.regs, &chip8.memory);
            let instruction = decode_instruction(raw_instruction);
            execute_instruction(&instruction, &mut chip8);
        }
        chip8.decrement_timers();

        // draw new screen state
        canvas.set_draw_color(COLOR_ON);
        for i in 0..SCREEN_WIDTH {
            for j in 0..SCREEN_HEIGHT {
                let x = i as u32 * DISP_SCALE;
                let y = j as u32 * DISP_SCALE;
                if chip8.get_px(i as u8, j as u8) > 0 {
                    canvas
                        .fill_rect(Rect::new(x as i32, y as i32, DISP_SCALE, DISP_SCALE))
                        .unwrap();
                }
            }
        }

        canvas.present();
        // keep emulation at ~60FPS (execution time of the loop not counted)
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn load_file(name: &str) -> Vec<u8> {
    return std::fs::read(["./roms/", name].join("")).unwrap();
}

fn keycode_to_button(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}
