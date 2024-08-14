struct Registers {
    general: [u8; 16],   // general purpose registers
    dt: u8,              // delay timer
    st: u8,              // sound timer
    pc: u16,             // program counter
    sp: u8,              // stack pointer
    i: u16              // index register
}

impl Registers {
    fn new() -> Registers {
        Registers {
            general: [0; 16],
            dt: 0,
            st: 0,
            pc: 0,
            sp: 0,
            i: 0
        }
    }
}

fn main() {
    println!("CHIP-8");
    println!("");
    println!("welcome to my CHIP-8 emulator :)");

    let mut registers = Registers::new();
    let mut stack: [u16; 16] = [0; 16];
    let mut memory: [u8; 4096] = [0; 4096];
    
    // quick test
    memory[0] = 1;
    memory[1] = 2;
    
    let next_instruction = fetch_instruction(&mut registers, &memory);
    println!("next instruction is {}", next_instruction);
}

fn fetch_instruction(registers: &mut Registers, memory: &[u8; 4096]) -> u16 {
    let high_byte = memory[registers.pc as usize] as u16;
    let low_byte = memory[registers.pc as usize + 1] as u16;
    registers.pc += 2;
    return high_byte << 8 | low_byte;
}
