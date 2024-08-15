const DISP_BUFFER_SIZE: usize = (64 / 8) * (32 / 8);
const STACK_SIZE: usize = 16;
const MEMORY_SIZE: usize = 4096;

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
    Cls,
    Ret,
    Jmp { address: u16 },
    Call { address: u16 },
    Se { reg: u8, val: u8 },
    Sne { reg: u8, val: u8 },
    SeReg { reg1: u8, reg2: u8 },
    Ld { reg: u8, val: u8 },
    Add { reg: u8, val: u8 },
    LdReg { reg1: u8, reg2: u8 },
    Or { reg1: u8, reg2: u8 },
    And { reg1: u8, reg2: u8 },
    Xor { reg1: u8, reg2: u8 },
    AddReg { reg1: u8, reg2: u8 },
    SubReg { reg1: u8, reg2: u8 },
    Shr { reg1: u8, reg2: u8 },
    SubN { reg1: u8, reg2: u8 },
    Shl { reg1: u8, reg2: u8 },
    SneReg { reg1: u8, reg2: u8 },
    Ldi { address: u16 },
    JmpV0 { address: u16 },
    Rnd { reg: u8, mask: u8 },
    Drw { reg1: u8, reg2: u8, n_bytes: u8 },
    Skp { key: u8 },
    SkpNp { key: u8 },
    LdFromDt { reg: u8 },
    LdKey { reg: u8 },
    LdIntoDt { reg: u8 },
    LdSt { reg: u8 },
    AddI { reg: u8 },
    LdF { reg: u8 },
    LdB { reg: u8 },
    LdRegsMem { reg: u8 },
    LdMemRegs { reg: u8 },
}

struct Chip8 {
    regs: Registers,
    disp_buffer: [u8; DISP_BUFFER_SIZE],
    stack: [u16; 16],
    memory: [u8; 4096],
}

impl Chip8 {
    fn new() -> Chip8 {
        Chip8 {
            regs: Registers::new(),
            disp_buffer: [0; DISP_BUFFER_SIZE],
            stack: [0; STACK_SIZE],
            memory: [0; MEMORY_SIZE],
        }
    }
}

struct Registers {
    general: [u8; 16], // general purpose registers
    dt: u8,            // delay timer
    st: u8,            // sound timer
    pc: u16,           // program counter
    sp: u8,            // stack pointer
    i: u16,            // index register
}

impl Registers {
    fn new() -> Registers {
        Registers {
            general: [0; 16],
            dt: 0,
            st: 0,
            pc: 0,
            sp: 0,
            i: 0,
        }
    }
}

fn main() {
    println!("CHIP-8");
    println!("");
    println!("welcome to my CHIP-8 emulator :)");

    let mut chip8 = Chip8::new();

    let next_instruction = fetch_instruction(&mut chip8.regs, &chip8.memory);
    println!("next instruction is {}", next_instruction);
}

fn fetch_instruction(registers: &mut Registers, memory: &[u8; 4096]) -> u16 {
    let high_byte = memory[registers.pc as usize] as u16;
    let low_byte = memory[registers.pc as usize + 1] as u16;
    registers.pc += 2;
    return high_byte << 8 | low_byte;
}

fn decode_instruction(instruction: u16) -> Instruction {
    match get_nibble_u16(instruction, 3) {
        0x0 => match instruction {
            0x00E0 => Instruction::Cls,
            0x00EE => Instruction::Ret,
            _ => panic!("Instruction #X{} not recognized", instruction),
        },
        0x1 => Instruction::Jmp {
            address: instruction & 0x0FFF,
        },
        0x2 => Instruction::Call {
            address: instruction & 0x0FFF,
        },
        0x3 => Instruction::Se {
            reg: get_nibble_u16(instruction, 2),
            val: (instruction & 0xFF) as u8,
        },
        0x4 => Instruction::Sne {
            reg: get_nibble_u16(instruction, 2),
            val: (instruction & 0xFF) as u8,
        },
        0x5 => Instruction::SeReg {
            reg1: get_nibble_u16(instruction, 2),
            reg2: get_nibble_u16(instruction, 1),
        },
        0x6 => Instruction::Ld {
            reg: get_nibble_u16(instruction, 2),
            val: (instruction & 0xFF) as u8,
        },
        0x7 => Instruction::Add {
            reg: get_nibble_u16(instruction, 2),
            val: (instruction & 0xFF) as u8,
        },
        0x8 => {
            let reg1 = get_nibble_u16(instruction, 2);
            let reg2 = get_nibble_u16(instruction, 1);
            match get_nibble_u16(instruction, 0) {
                0x0 => Instruction::LdReg { reg1, reg2 },
                0x1 => Instruction::Or { reg1, reg2 },
                0x2 => Instruction::And { reg1, reg2 },
                0x3 => Instruction::Xor { reg1, reg2 },
                0x4 => Instruction::AddReg { reg1, reg2 },
                0x5 => Instruction::SubReg { reg1, reg2 },
                0x6 => Instruction::Shr { reg1, reg2 },
                0x7 => Instruction::SubN { reg1, reg2 },
                0xE => Instruction::Shl { reg1, reg2 },
                unknown_op => panic!("Opcode #X{} not recognized for group 8", unknown_op),
            }
        }
        0x9 => Instruction::SneReg {
            reg1: get_nibble_u16(instruction, 2),
            reg2: get_nibble_u16(instruction, 1),
        },
        0xA => Instruction::Ldi {
            address: (instruction & 0x0FFF) as u16,
        },
        0xB => Instruction::JmpV0 {
            address: (instruction & 0x0FFF) as u16,
        },
        0xC => Instruction::Rnd {
            reg: get_nibble_u16(instruction, 2),
            mask: (instruction & 0xFF) as u8,
        },
        0xD => Instruction::Drw {
            reg1: get_nibble_u16(instruction, 2),
            reg2: get_nibble_u16(instruction, 1),
            n_bytes: get_nibble_u16(instruction, 0),
        },
        0xE => {
            let key = get_nibble_u16(instruction, 2);
            match (instruction & 0xFF) as u8 {
                0x9E => Instruction::Skp { key },
                0xA1 => Instruction::SkpNp { key },
                unknown_op => panic!("Opcode #X{} not recognized for group E", unknown_op),
            }
        }
        0xF => {
            let reg = get_nibble_u16(instruction, 2);
            match (instruction & 0xFF) as u8 {
                0x07 => Instruction::LdFromDt { reg },
                0x0A => Instruction::LdKey { reg },
                0x15 => Instruction::LdIntoDt { reg },
                0x18 => Instruction::LdSt { reg },
                0x1E => Instruction::AddI { reg },
                0x29 => Instruction::LdF { reg },
                0x33 => Instruction::LdB { reg },
                0x55 => Instruction::LdRegsMem { reg },
                0x65 => Instruction::LdMemRegs { reg },
                unknown_op => panic!("Opcode #X{} not recognized for group 8", unknown_op),
            }
        }
        _ => panic!("Instruction #X{} not recognized", instruction),
    }
}

fn execute_instruction(ins: &Instruction, chip8: &mut Chip8) {
    match ins {
        Instruction::Cls => {
            for byte in chip8.disp_buffer.iter_mut() {
                *byte = 0;
            }
        }
        _ => panic!("Not implemented!"),
    }
}

/// Gets the i-th nibble (half-byte) from x
/// # Example
/// ```
/// get_nibble_u16(0xABCD, 0) == 0x0D
/// ```
fn get_nibble_u16(x: u16, i: u8) -> u8 {
    return ((x >> i * 4) & 0xF) as u8;
}

#[cfg(test)]
mod tests {
    use crate::{decode_instruction, execute_instruction, get_nibble_u16, Chip8, Instruction};

    #[test]
    fn get_nibble_u16_works() {
        assert_eq!(get_nibble_u16(0x4321, 0), 0x01);
        assert_eq!(get_nibble_u16(0x4321, 1), 0x02);
        assert_eq!(get_nibble_u16(0x4321, 2), 0x03);
        assert_eq!(get_nibble_u16(0x4321, 3), 0x04);
    }

    #[test]
    fn decode_instruction_works() {
        let cases = [
            (0x00E0, Instruction::Cls),
            (0x00EE, Instruction::Ret),
            (0x1ABC, Instruction::Jmp { address: 0xABC }),
            (0x2ABC, Instruction::Call { address: 0xABC }),
            (
                0x31AB,
                Instruction::Se {
                    reg: 0x1,
                    val: 0xAB,
                },
            ),
            (
                0x41AB,
                Instruction::Sne {
                    reg: 0x1,
                    val: 0xAB,
                },
            ),
            (
                0x5120,
                Instruction::SeReg {
                    reg1: 0x1,
                    reg2: 0x2,
                },
            ),
            (
                0x61AB,
                Instruction::Ld {
                    reg: 0x1,
                    val: 0xAB,
                },
            ),
            (
                0x71AB,
                Instruction::Add {
                    reg: 0x1,
                    val: 0xAB,
                },
            ),
            (
                0x8AB0,
                Instruction::LdReg {
                    reg1: 0xA,
                    reg2: 0xB,
                },
            ),
            (
                0x8AB1,
                Instruction::Or {
                    reg1: 0xA,
                    reg2: 0xB,
                },
            ),
            (
                0x8AB2,
                Instruction::And {
                    reg1: 0xA,
                    reg2: 0xB,
                },
            ),
            (
                0x8AB3,
                Instruction::Xor {
                    reg1: 0xA,
                    reg2: 0xB,
                },
            ),
            (
                0x8AB4,
                Instruction::AddReg {
                    reg1: 0xA,
                    reg2: 0xB,
                },
            ),
            (
                0x8AB5,
                Instruction::SubReg {
                    reg1: 0xA,
                    reg2: 0xB,
                },
            ),
            (
                0x8AB6,
                Instruction::Shr {
                    reg1: 0xA,
                    reg2: 0xB,
                },
            ),
            (
                0x8AB7,
                Instruction::SubN {
                    reg1: 0xA,
                    reg2: 0xB,
                },
            ),
            (
                0x8ABE,
                Instruction::Shl {
                    reg1: 0xA,
                    reg2: 0xB,
                },
            ),
            (
                0x9AB0,
                Instruction::SneReg {
                    reg1: 0xA,
                    reg2: 0xB,
                },
            ),
            (0xAABC, Instruction::Ldi { address: 0xABC }),
            (0xBABC, Instruction::JmpV0 { address: 0xABC }),
            (
                0xC1AB,
                Instruction::Rnd {
                    reg: 0x1,
                    mask: 0xAB,
                },
            ),
            (
                0xDAB5,
                Instruction::Drw {
                    reg1: 0xA,
                    reg2: 0xB,
                    n_bytes: 0x5,
                },
            ),
            (0xE19E, Instruction::Skp { key: 0x1 }),
            (0xE2A1, Instruction::SkpNp { key: 0x2 }),
            (0xF107, Instruction::LdFromDt { reg: 0x1 }),
            (0xF10A, Instruction::LdKey { reg: 0x1 }),
            (0xF115, Instruction::LdIntoDt { reg: 0x1 }),
            (0xF118, Instruction::LdSt { reg: 0x1 }),
            (0xF11E, Instruction::AddI { reg: 0x1 }),
            (0xF129, Instruction::LdF { reg: 0x1 }),
            (0xF133, Instruction::LdB { reg: 0x1 }),
            (0xF155, Instruction::LdRegsMem { reg: 0x1 }),
            (0xF165, Instruction::LdMemRegs { reg: 0x1 }),
        ];

        for (input, expected) in cases.iter() {
            assert_eq!(decode_instruction(*input), *expected);
        }
    }

    #[test]
    fn execute_cls_works() {
        let mut chip8 = Chip8::new();
        for (i, byte) in chip8.disp_buffer.iter_mut().enumerate() {
            *byte = (i % 0xFF) as u8;
        }
        chip8.disp_buffer[0] = 0xFF;

        execute_instruction(&Instruction::Cls, &mut chip8);

        for byte in chip8.disp_buffer.iter() {
            assert_eq!(*byte, 0u8)
        }
    }
}
