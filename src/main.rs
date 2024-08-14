#[derive(PartialEq, Eq, Debug)]
enum Instruction {
    Cls,
    Ret,
    Jmp { address: u16 },
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
    return match instruction {
        0x00E0 => Instruction::Cls,
        0x00EE => Instruction::Ret,
        x if get_nibble_u16(x, 3) == 0x1 => Instruction::Jmp {
            address: 0x0FFF & x,
        },
        _ => panic!("Instruction #X{} not recognized", instruction),
    };
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
}
