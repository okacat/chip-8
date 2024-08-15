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
    Rnd { mask: u8 },
    Drw { reg1: u8, reg2: u8, n_bytes: u8 },
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
        _ => panic!("Instruction #X{} not recognized", instruction),
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

// 0000 0000 0000 0000
// F

#[cfg(test)]
mod tests {
    use crate::{decode_instruction, get_nibble_u16, Instruction};

    #[test]
    fn get_nibble_works() {
        assert_eq!(get_nibble_u16(0x4321, 0), 0x01);
        assert_eq!(get_nibble_u16(0x4321, 1), 0x02);
        assert_eq!(get_nibble_u16(0x4321, 2), 0x03);
        assert_eq!(get_nibble_u16(0x4321, 3), 0x04);
    }

    #[test]
    fn decode_cls() {
        assert_eq!(decode_instruction(0x00E0), Instruction::Cls);
    }

    #[test]
    fn decode_ret() {
        assert_eq!(decode_instruction(0x00EE), Instruction::Ret);
    }

    #[test]
    fn decode_jmp() {
        assert_eq!(
            decode_instruction(0x1ABC),
            Instruction::Jmp { address: 0xABC }
        );
    }

    #[test]
    fn decode_call() {
        assert_eq!(
            decode_instruction(0x2ABC),
            Instruction::Call { address: 0xABC }
        );
    }

    #[test]
    fn decode_skip_equal() {
        assert_eq!(
            decode_instruction(0x31AB),
            Instruction::Se {
                reg: 0x1,
                val: 0xAB
            }
        );
    }

    #[test]
    fn decode_skip_not_equal() {
        assert_eq!(
            decode_instruction(0x41AB),
            Instruction::Sne {
                reg: 0x1,
                val: 0xAB
            }
        );
    }

    #[test]
    fn decode_skip_equal_registers() {
        assert_eq!(
            decode_instruction(0x5120),
            Instruction::SeReg {
                reg1: 0x1,
                reg2: 0x2
            }
        );
    }

    #[test]
    fn decode_load() {
        assert_eq!(
            decode_instruction(0x61AB),
            Instruction::Ld {
                reg: 0x1,
                val: 0xAB
            }
        );
    }

    #[test]
    fn decode_add() {
        assert_eq!(
            decode_instruction(0x71AB),
            Instruction::Add {
                reg: 0x1,
                val: 0xAB
            }
        );
    }

    #[test]
    fn decode_load_reg() {
        assert_eq!(
            decode_instruction(0x8AB0),
            Instruction::LdReg {
                reg1: 0xA,
                reg2: 0xB
            }
        );
    }

    #[test]
    fn decode_or() {
        assert_eq!(
            decode_instruction(0x8AB1),
            Instruction::Or {
                reg1: 0xA,
                reg2: 0xB
            }
        );
    }

    #[test]
    fn decode_and() {
        assert_eq!(
            decode_instruction(0x8AB2),
            Instruction::And {
                reg1: 0xA,
                reg2: 0xB
            }
        );
    }

    #[test]
    fn decode_load_xor() {
        assert_eq!(
            decode_instruction(0x8AB3),
            Instruction::Xor {
                reg1: 0xA,
                reg2: 0xB
            }
        );
    }

    #[test]
    fn decode_load_add_reg() {
        assert_eq!(
            decode_instruction(0x8AB4),
            Instruction::AddReg {
                reg1: 0xA,
                reg2: 0xB
            }
        );
    }

    #[test]
    fn decode_sub_reg() {
        assert_eq!(
            decode_instruction(0x8AB5),
            Instruction::SubReg {
                reg1: 0xA,
                reg2: 0xB
            }
        );
    }

    #[test]
    fn decode_shr() {
        assert_eq!(
            decode_instruction(0x8AB6),
            Instruction::Shr {
                reg1: 0xA,
                reg2: 0xB
            }
        );
    }

    #[test]
    fn decode_subn() {
        assert_eq!(
            decode_instruction(0x8AB7),
            Instruction::SubN {
                reg1: 0xA,
                reg2: 0xB
            }
        );
    }

    #[test]
    fn decode_shl() {
        assert_eq!(
            decode_instruction(0x8ABE),
            Instruction::Shl {
                reg1: 0xA,
                reg2: 0xB
            }
        );
    }

    #[test]
    fn decode_skip_not_equal_registers() {
        assert_eq!(
            decode_instruction(0x9AB0),
            Instruction::SneReg {
                reg1: 0xA,
                reg2: 0xB
            }
        );
    }

    #[test]
    fn decode_load_i() {
        assert_eq!(
            decode_instruction(0xAABC),
            Instruction::Ldi { address: 0xABC }
        );
    }
}
