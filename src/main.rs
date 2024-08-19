mod chip8;

use chip8::{decode_instruction, execute_instruction, fetch_instruction, Chip8};

fn main() {
    println!("CHIP-8");
    println!("");
    println!("welcome to my CHIP-8 emulator :)");

    let mut chip8 = Chip8::new();

    let rom = load_file("test_opcode.ch8");
    chip8.load_into_mem(&rom, 0x200);
    chip8.regs.pc = 0x200;

    loop {
        let raw_instruction = fetch_instruction(&mut chip8.regs, &chip8.memory);
        let instruction = decode_instruction(raw_instruction);
        execute_instruction(&instruction, &mut chip8);
        chip8.dbg_print_display();
    }
}

fn load_file(name: &str) -> Vec<u8> {
    return std::fs::read(["./roms/", name].join("")).unwrap();
}
