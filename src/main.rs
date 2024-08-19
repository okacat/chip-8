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

    let mut chip8 = Chip8::new();

    // let rom = load_file("test_opcode.ch8");
    let rom = load_file("IBM Logo.ch8");
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
                _ => {}
            }
        }

        // clear screen
        canvas.set_draw_color(COLOR_OFF);
        canvas.clear();

        // do one step of chip-8
        let raw_instruction = fetch_instruction(&mut chip8.regs, &chip8.memory);
        let instruction = decode_instruction(raw_instruction);
        execute_instruction(&instruction, &mut chip8);

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
        // keep emulation at ~60FPS
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn load_file(name: &str) -> Vec<u8> {
    return std::fs::read(["./roms/", name].join("")).unwrap();
}
