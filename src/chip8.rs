use fastrand;

pub const SCREEN_WIDTH: u8 = 64;
pub const SCREEN_HEIGHT: u8 = 32;
const DISP_BUFFER_SIZE: usize = SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize;
const STACK_SIZE: usize = 16;
const MEMORY_SIZE: usize = 4096;

pub struct Chip8 {
    pub regs: Registers,
    pub disp_buffer: [u8; DISP_BUFFER_SIZE],
    pub stack: [u16; 16],
    pub memory: [u8; 4096],
    pub key_down: [bool; 16],
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            regs: Registers::new(),
            disp_buffer: [0; DISP_BUFFER_SIZE],
            stack: [0; STACK_SIZE],
            memory: [0; MEMORY_SIZE],
            key_down: [false; 16],
        }
    }

    pub fn decrement_timers(&mut self) {
        if self.regs.dt > 0 {
            self.regs.dt -= 1
        };

        if self.regs.st > 0 {
            self.regs.st -= 1
        };
    }

    pub fn get_px(&self, x: u8, y: u8) -> u8 {
        return self.disp_buffer[y as usize * SCREEN_WIDTH as usize + x as usize];
    }

    pub fn set_px(&mut self, x: u8, y: u8, val: u8) {
        return self.disp_buffer[y as usize * SCREEN_WIDTH as usize + x as usize] = val;
    }

    pub fn dbg_print_display(&self) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                print!("{} ", if self.get_px(x, y) > 0 { "O" } else { "." });
            }
            println!()
        }
        println!()
    }

    pub fn load_into_mem(&mut self, data: &Vec<u8>, start_addr: u16) {
        for (i, byte) in data.iter().enumerate() {
            self.memory[start_addr as usize + i] = *byte;
        }
    }

    pub fn load_font(&mut self) {
        let font = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        for (i, byte) in font.iter().enumerate() {
            self.memory[i] = *byte;
        }
    }
}

pub struct Registers {
    pub general: [u8; 16], // general purpose registers
    pub dt: u8,            // delay timer
    pub st: u8,            // sound timer
    pub pc: u16,           // program counter
    pub sp: u8,            // stack pointer
    pub i: u16,            // index register
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

#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
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
    SubRegN { reg1: u8, reg2: u8 },
    Shl { reg1: u8, reg2: u8 },
    SneReg { reg1: u8, reg2: u8 },
    Ldi { address: u16 },
    JmpV0 { address: u16 },
    Rnd { reg: u8, mask: u8 },
    Drw { reg1: u8, reg2: u8, n_bytes: u8 },
    Skp { reg: u8 },
    SkpNp { reg: u8 },
    LdFromDt { reg: u8 },
    LdKey { reg: u8 },
    LdIntoDt { reg: u8 },
    LdSt { reg: u8 },
    AddI { reg: u8 },
    LdF { reg: u8 },
    LdB { reg: u8 },
    LdRegsMem { end_reg: u8 },
    LdMemRegs { end_reg: u8 },
}

pub fn fetch_instruction(registers: &mut Registers, memory: &[u8; 4096]) -> u16 {
    let high_byte = memory[registers.pc as usize] as u16;
    let low_byte = memory[registers.pc as usize + 1] as u16;
    registers.pc += 2;
    return high_byte << 8 | low_byte;
}

pub fn decode_instruction(instruction: u16) -> Instruction {
    match get_nibble_u16(instruction, 3) {
        0x0 => match instruction {
            0x00E0 => Instruction::Cls,
            0x00EE => Instruction::Ret,
            _ => panic!("Instruction {} not recognized", instruction),
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
                0x7 => Instruction::SubRegN { reg1, reg2 },
                0xE => Instruction::Shl { reg1, reg2 },
                unknown_op => panic!("Opcode {} not recognized for group 8", unknown_op),
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
            let reg = get_nibble_u16(instruction, 2);
            match (instruction & 0xFF) as u8 {
                0x9E => Instruction::Skp { reg },
                0xA1 => Instruction::SkpNp { reg },
                unknown_op => panic!("Opcode {} not recognized for group E", unknown_op),
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
                0x55 => Instruction::LdRegsMem { end_reg: reg },
                0x65 => Instruction::LdMemRegs { end_reg: reg },
                unknown_op => panic!("Opcode {} not recognized for group 8", unknown_op),
            }
        }
        _ => panic!("Instruction {} not recognized", instruction),
    }
}

pub fn execute_instruction(ins: &Instruction, chip8: &mut Chip8) {
    match ins {
        Instruction::Cls => {
            for byte in chip8.disp_buffer.iter_mut() {
                *byte = 0;
            }
        }
        Instruction::Ret => {
            let address = chip8.stack[chip8.regs.sp as usize];
            chip8.regs.pc = address;
            if chip8.regs.sp > 0 {
                chip8.regs.sp -= 1;
            }
        }
        Instruction::Jmp { address } => chip8.regs.pc = *address,
        Instruction::Call { address } => {
            chip8.regs.sp += 1;
            chip8.stack[chip8.regs.sp as usize] = chip8.regs.pc;
            chip8.regs.pc = *address;
        }
        Instruction::Se { reg, val } => {
            if chip8.regs.general[*reg as usize] == *val {
                chip8.regs.pc += 2;
            }
        }
        Instruction::Sne { reg, val } => {
            if chip8.regs.general[*reg as usize] != *val {
                chip8.regs.pc += 2;
            }
        }
        Instruction::SeReg { reg1, reg2 } => {
            if chip8.regs.general[*reg1 as usize] == chip8.regs.general[*reg2 as usize] {
                chip8.regs.pc += 2;
            }
        }
        Instruction::Ld { reg, val } => chip8.regs.general[*reg as usize] = *val,
        Instruction::Add { reg, val } => {
            let reg_val = chip8.regs.general[*reg as usize];
            let result = reg_val as u32 + *val as u32;
            chip8.regs.general[*reg as usize] = result as u8;
        }
        Instruction::LdReg { reg1, reg2 } => {
            chip8.regs.general[*reg1 as usize] = chip8.regs.general[*reg2 as usize]
        }
        Instruction::Or { reg1, reg2 } => {
            let result = chip8.regs.general[*reg1 as usize] | chip8.regs.general[*reg2 as usize];
            chip8.regs.general[*reg1 as usize] = result;
        }
        Instruction::And { reg1, reg2 } => {
            let result = chip8.regs.general[*reg1 as usize] & chip8.regs.general[*reg2 as usize];
            chip8.regs.general[*reg1 as usize] = result;
        }
        Instruction::Xor { reg1, reg2 } => {
            let result = chip8.regs.general[*reg1 as usize] ^ chip8.regs.general[*reg2 as usize];
            chip8.regs.general[*reg1 as usize] = result;
        }
        Instruction::AddReg { reg1, reg2 } => {
            let reg1_val = chip8.regs.general[*reg1 as usize] as u32;
            let reg2_val = chip8.regs.general[*reg2 as usize] as u32;
            let result = reg1_val + reg2_val;
            chip8.regs.general[0xF] = if result > 0xFF { 0x1 } else { 0x0 };
            chip8.regs.general[*reg1 as usize] = result as u8;
        }
        Instruction::SubReg { reg1, reg2 } => {
            let reg1_val = chip8.regs.general[*reg1 as usize] as i32;
            let reg2_val = chip8.regs.general[*reg2 as usize] as i32;
            chip8.regs.general[0xF] = if reg1_val > reg2_val { 0x1 } else { 0x0 };
            let result = (reg1_val - reg2_val) as u8;

            chip8.regs.general[*reg1 as usize] = result as u8;
        }
        Instruction::Shr { reg1, .. } => {
            // TODO: CHIP-48 and SUPER-CHIP also do VX = VY first
            let reg1_val = chip8.regs.general[*reg1 as usize];
            chip8.regs.general[0xF] = reg1_val & 0x01;
            chip8.regs.general[*reg1 as usize] = reg1_val / 2;
        }
        Instruction::Shl { reg1, .. } => {
            // TODO: CHIP-48 and SUPER-CHIP also do VX = VY first
            let reg1_val = chip8.regs.general[*reg1 as usize];
            chip8.regs.general[0xF] = if reg1_val & 0x80 > 0 { 0x1 } else { 0x0 };
            chip8.regs.general[*reg1 as usize] = ((reg1_val as u16) * 2) as u8;
        }
        Instruction::SubRegN { reg1, reg2 } => {
            let reg1_val = chip8.regs.general[*reg1 as usize] as i32;
            let reg2_val = chip8.regs.general[*reg2 as usize] as i32;
            chip8.regs.general[0xF] = if reg2_val > reg1_val { 0x1 } else { 0x0 };
            let result: u8 = (reg2_val - reg1_val) as u8;

            chip8.regs.general[*reg1 as usize] = result as u8;
        }
        Instruction::SneReg { reg1, reg2 } => {
            if chip8.regs.general[*reg1 as usize] != chip8.regs.general[*reg2 as usize] {
                chip8.regs.pc += 2;
            }
        }
        Instruction::Ldi { address } => chip8.regs.i = *address,
        Instruction::JmpV0 { address } => {
            chip8.regs.pc = chip8.regs.general[0] as u16 + *address;
        }
        Instruction::Rnd { reg, mask } => {
            let rnd_val = fastrand::u8(..);
            let result = rnd_val as u8 & *mask;
            chip8.regs.general[*reg as usize] = result;
        }
        Instruction::Drw {
            reg1,
            reg2,
            n_bytes,
        } => {
            let x = chip8.regs.general[*reg1 as usize] % SCREEN_WIDTH;
            let y = chip8.regs.general[*reg2 as usize] % SCREEN_HEIGHT;
            chip8.regs.general[0xF] = 0;

            for row in 0..*n_bytes as usize {
                let sprite_row = chip8.memory[chip8.regs.i as usize + row];
                let cy = y + row as u8;
                if cy >= SCREEN_HEIGHT {
                    break;
                };
                for bit_i in 0..8 {
                    let cx = x + bit_i;
                    if cx >= SCREEN_WIDTH {
                        break;
                    }
                    let sprite_bit = if sprite_row & 0x80 >> bit_i > 0 {
                        0x1
                    } else {
                        0x0
                    };
                    let disp_bit = chip8.get_px(cx, cy);
                    if sprite_bit > 0 && disp_bit > 0 {
                        chip8.regs.general[0xF] = 0x1;
                    }
                    chip8.set_px(cx, cy, disp_bit ^ sprite_bit);
                }
            }
        }
        Instruction::Skp { reg } => {
            let key = chip8.regs.general[*reg as usize];
            if chip8.key_down[key as usize] {
                chip8.regs.pc += 2;
            }
        }
        Instruction::SkpNp { reg } => {
            let key = chip8.regs.general[*reg as usize];
            if !chip8.key_down[key as usize] {
                chip8.regs.pc += 2;
            }
        }
        Instruction::LdFromDt { reg } => {
            chip8.regs.general[*reg as usize] = chip8.regs.dt;
        }
        Instruction::LdIntoDt { reg } => {
            chip8.regs.dt = chip8.regs.general[*reg as usize];
        }
        Instruction::LdKey { reg } => {
            let mut was_key_pressed = false;
            for (key, is_down) in chip8.key_down.iter().enumerate() {
                if *is_down {
                    chip8.regs.general[*reg as usize] = key as u8;
                    was_key_pressed = true;
                    break;
                }
            }
            // repeat instruction if no key was pressed
            if !was_key_pressed {
                chip8.regs.pc -= 2;
            }
        }
        Instruction::LdSt { reg } => chip8.regs.st = chip8.regs.general[*reg as usize],
        Instruction::AddI { reg } => chip8.regs.i += chip8.regs.general[*reg as usize] as u16,
        Instruction::LdF { reg } => chip8.regs.i = chip8.regs.general[*reg as usize] as u16 * 0x5,
        Instruction::LdB { reg } => {
            let val = chip8.regs.general[*reg as usize];
            let hundreds = val / 100;
            let tens = (val - hundreds * 100) / 10;
            let ones = val % 10;
            let i = chip8.regs.i as usize;
            chip8.memory[i] = hundreds;
            chip8.memory[i + 1] = tens;
            chip8.memory[i + 2] = ones;
        }
        Instruction::LdRegsMem { end_reg } => {
            for reg in 0..=*end_reg as usize {
                let addr = chip8.regs.i as usize + reg as usize;
                chip8.memory[addr] = chip8.regs.general[reg];
            }
        }
        Instruction::LdMemRegs { end_reg } => {
            for reg in 0..=*end_reg as usize {
                let addr = chip8.regs.i as usize + reg as usize;
                chip8.regs.general[reg] = chip8.memory[addr];
            }
        }
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
    use super::*;

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
                Instruction::SubRegN {
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
            (0xE19E, Instruction::Skp { reg: 0x1 }),
            (0xE2A1, Instruction::SkpNp { reg: 0x2 }),
            (0xF107, Instruction::LdFromDt { reg: 0x1 }),
            (0xF10A, Instruction::LdKey { reg: 0x1 }),
            (0xF115, Instruction::LdIntoDt { reg: 0x1 }),
            (0xF118, Instruction::LdSt { reg: 0x1 }),
            (0xF11E, Instruction::AddI { reg: 0x1 }),
            (0xF129, Instruction::LdF { reg: 0x1 }),
            (0xF133, Instruction::LdB { reg: 0x1 }),
            (0xF155, Instruction::LdRegsMem { end_reg: 0x1 }),
            (0xF165, Instruction::LdMemRegs { end_reg: 0x1 }),
        ];

        for (input, expected) in cases.iter() {
            assert_eq!(decode_instruction(*input), *expected);
        }
    }

    #[test]
    fn execute_cls_works() {
        let mut chip8 = Chip8::new();
        for byte in chip8.disp_buffer.iter_mut() {
            *byte = 0xFF;
        }

        execute_instruction(&Instruction::Cls, &mut chip8);

        for byte in chip8.disp_buffer.iter() {
            assert_eq!(*byte, 0u8);
        }
    }

    #[test]
    fn execute_ret_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.sp = 0;
        chip8.stack[0] = 0x0ABC;

        execute_instruction(&Instruction::Ret, &mut chip8);
        assert_eq!(chip8.regs.pc, 0xABC);
        assert_eq!(chip8.regs.sp, 0)
    }

    #[test]
    fn execute_jmp_works() {
        let mut chip8 = Chip8::new();

        execute_instruction(&Instruction::Jmp { address: 0xABC }, &mut chip8);
        assert_eq!(chip8.regs.pc, 0xABC);
    }

    #[test]
    fn execute_call_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.pc = 0x123;

        execute_instruction(&Instruction::Call { address: 0xABC }, &mut chip8);
        assert_eq!(chip8.regs.pc, 0xABC);
        assert_eq!(chip8.regs.sp, 1);
        assert_eq!(chip8.stack[1], 0x123);
    }

    #[test]
    fn execute_se_and_sne_works() {
        let mut chip8 = Chip8::new();

        execute_instruction(&&Instruction::Se { reg: 0x0, val: 0x0 }, &mut chip8);
        assert_eq!(chip8.regs.pc, 0x2);

        execute_instruction(&&Instruction::Se { reg: 0x0, val: 0x1 }, &mut chip8);
        assert_eq!(chip8.regs.pc, 0x2);

        execute_instruction(&&Instruction::Sne { reg: 0x0, val: 0x1 }, &mut chip8);
        assert_eq!(chip8.regs.pc, 0x4);

        execute_instruction(&&Instruction::Sne { reg: 0x0, val: 0x0 }, &mut chip8);
        assert_eq!(chip8.regs.pc, 0x4);
    }

    #[test]
    fn execute_se_reg_and_sne_reg_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.general[0] = 0xA;
        chip8.regs.general[1] = 0xB;
        chip8.regs.general[2] = 0xA;

        execute_instruction(
            &&Instruction::SeReg {
                reg1: 0x0,
                reg2: 0x2,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.pc, 0x2);

        execute_instruction(
            &&Instruction::SeReg {
                reg1: 0x0,
                reg2: 0x1,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.pc, 0x2);

        execute_instruction(
            &&Instruction::SneReg {
                reg1: 0x0,
                reg2: 0x1,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.pc, 0x4);

        execute_instruction(
            &&Instruction::SneReg {
                reg1: 0x0,
                reg2: 0x2,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.pc, 0x4);
    }

    #[test]
    fn execute_ld_works() {
        let mut chip8 = Chip8::new();

        execute_instruction(
            &Instruction::Ld {
                reg: 0xA,
                val: 0x1F,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0x1F);
    }

    #[test]
    fn execute_add_works() {
        let mut chip8 = Chip8::new();

        // no overflow
        chip8.regs.general[0xA] = 0x5;

        execute_instruction(
            &Instruction::Add {
                reg: 0xA,
                val: 0x10,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0x15);

        // overflow
        chip8.regs.general[0xA] = 0xFF;

        execute_instruction(&Instruction::Add { reg: 0xA, val: 0x2 }, &mut chip8);
        assert_eq!(chip8.regs.general[0xA], 0x01);
    }

    #[test]
    fn execute_ld_reg_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.general[0xA] = 0x5;
        chip8.regs.general[0xB] = 0x10;

        execute_instruction(
            &Instruction::LdReg {
                reg1: 0xA,
                reg2: 0xB,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0x10);
    }

    #[test]
    fn execute_or_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.general[0xA] = 0xC0;
        chip8.regs.general[0xB] = 0x0D;

        execute_instruction(
            &Instruction::Or {
                reg1: 0xA,
                reg2: 0xB,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0xCD);
    }

    #[test]
    fn execute_and_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.general[0xA] = 0x0F;
        chip8.regs.general[0xB] = 0xAA;

        execute_instruction(
            &Instruction::And {
                reg1: 0xA,
                reg2: 0xB,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0x0A);
    }

    #[test]
    fn execute_xor_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.general[0xA] = 0x0F;
        chip8.regs.general[0xB] = 0xAA;

        execute_instruction(
            &Instruction::Xor {
                reg1: 0xA,
                reg2: 0xB,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0xA5);
    }

    #[test]
    fn execute_add_reg_works() {
        let mut chip8 = Chip8::new();

        // no overflow
        chip8.regs.general[0xA] = 0x11;
        chip8.regs.general[0xB] = 0x05;

        execute_instruction(
            &Instruction::AddReg {
                reg1: 0xA,
                reg2: 0xB,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0x16);
        assert_eq!(chip8.regs.general[0xF], 0x0);

        // overflow
        chip8.regs.general[0xA] = 0xFF;
        chip8.regs.general[0xB] = 0x02;

        execute_instruction(
            &Instruction::AddReg {
                reg1: 0xA,
                reg2: 0xB,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0x01);
        assert_eq!(chip8.regs.general[0xF], 0x1);
    }

    #[test]
    fn execute_sub_reg_works() {
        let mut chip8 = Chip8::new();

        // no overflow
        chip8.regs.general[0xA] = 0x09;
        chip8.regs.general[0xB] = 0x04;

        execute_instruction(
            &Instruction::SubReg {
                reg1: 0xA,
                reg2: 0xB,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0x05);
        assert_eq!(chip8.regs.general[0xF], 0x1);

        // overflow
        chip8.regs.general[0xA] = 0x00;
        chip8.regs.general[0xB] = 0x02;

        execute_instruction(
            &Instruction::SubReg {
                reg1: 0xA,
                reg2: 0xB,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0xFE);
        assert_eq!(chip8.regs.general[0xF], 0x0);
    }

    #[test]
    fn execute_sub_reg_n_works() {
        let mut chip8 = Chip8::new();

        // no overflow
        chip8.regs.general[0xA] = 0x04;
        chip8.regs.general[0xB] = 0x09;

        execute_instruction(
            &Instruction::SubRegN {
                reg1: 0xA,
                reg2: 0xB,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0x05);
        assert_eq!(chip8.regs.general[0xF], 0x1);

        // overflow
        chip8.regs.general[0xA] = 0x02;
        chip8.regs.general[0xB] = 0x00;

        execute_instruction(
            &Instruction::SubRegN {
                reg1: 0xA,
                reg2: 0xB,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0xFE);
        assert_eq!(chip8.regs.general[0xF], 0x0);
    }

    #[test]
    fn execute_shr_works() {
        let mut chip8 = Chip8::new();

        // no overflow
        chip8.regs.general[0xA] = 0x8;

        execute_instruction(
            &Instruction::Shr {
                reg1: 0xA,
                reg2: 0x0,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0x04);
        assert_eq!(chip8.regs.general[0xF], 0x0);

        // overflow
        chip8.regs.general[0xA] = 0x5;

        execute_instruction(
            &Instruction::Shr {
                reg1: 0xA,
                reg2: 0x0,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0x02);
        assert_eq!(chip8.regs.general[0xF], 0x1);
    }

    #[test]
    fn execute_shl_works() {
        let mut chip8 = Chip8::new();

        // no overflow
        chip8.regs.general[0xA] = 0x4;

        execute_instruction(
            &Instruction::Shl {
                reg1: 0xA,
                reg2: 0x0,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0x08);
        assert_eq!(chip8.regs.general[0xF], 0x0);

        // overflow
        chip8.regs.general[0xA] = 0x81;

        execute_instruction(
            &Instruction::Shl {
                reg1: 0xA,
                reg2: 0x0,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0x2);
        assert_eq!(chip8.regs.general[0xF], 0x1);
    }

    #[test]
    fn execute_ldi_works() {
        let mut chip8 = Chip8::new();

        execute_instruction(&Instruction::Ldi { address: 0xABC }, &mut chip8);
        assert_eq!(chip8.regs.i, 0xABC);
    }

    #[test]
    fn execute_jmp_v0_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.general[0] = 0x5;

        execute_instruction(&Instruction::JmpV0 { address: 0xABC }, &mut chip8);
        assert_eq!(chip8.regs.pc, 0xAC1);
    }

    #[test]
    fn execute_rnd_works() {
        let mut chip8 = Chip8::new();

        fastrand::seed(42);
        // get the first 2 random numbers
        // assert_eq!(fastrand::u8(..), 0x89);
        // assert_eq!(fastrand::u8(..), 0xC6);

        execute_instruction(
            &Instruction::Rnd {
                reg: 0xA,
                mask: 0xFF,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xA], 0x89);

        execute_instruction(
            &Instruction::Rnd {
                reg: 0xB,
                mask: 0x0F,
            },
            &mut chip8,
        );
        assert_eq!(chip8.regs.general[0xB], 0x06);
    }

    #[test]
    fn execute_drw_works() {
        let mut chip8 = Chip8::new();

        let spr_addr = 0x200;
        chip8.regs.i = spr_addr;
        chip8.memory[0x200] = 0b11111111;
        chip8.memory[0x201] = 0b11111111;
        chip8.memory[0x202] = 0b11111111;
        chip8.regs.general[0xA] = 10;
        chip8.regs.general[0xB] = 5;

        // draw sprite
        execute_instruction(
            &Instruction::Drw {
                reg1: 0xA,
                reg2: 0xB,
                n_bytes: 3,
            },
            &mut chip8,
        );

        chip8.dbg_print_display();
        for x in 10..18 {
            for y in 5..8 {
                assert_eq!(chip8.get_px(x, y), 0x1);
            }
        }
        assert_eq!(chip8.regs.general[0xF], 0x0);

        // re-draw sprite over the old one, it should clear the display
        execute_instruction(
            &Instruction::Drw {
                reg1: 0xA,
                reg2: 0xB,
                n_bytes: 3,
            },
            &mut chip8,
        );
        chip8.dbg_print_display();
        for byte in chip8.disp_buffer.iter() {
            assert_eq!(*byte, 0x0);
        }
        assert_eq!(chip8.regs.general[0xF], 0x1);

        // draw the sprite at x and y higher than screen coord, it should wrap
        chip8.regs.general[0xA] = 10 + 64;
        chip8.regs.general[0xB] = 5 + 32;
        execute_instruction(
            &Instruction::Drw {
                reg1: 0xA,
                reg2: 0xB,
                n_bytes: 3,
            },
            &mut chip8,
        );
        chip8.dbg_print_display();
        for x in 10..18 {
            for y in 5..8 {
                assert_eq!(chip8.get_px(x, y), 0x1);
            }
        }
        assert_eq!(chip8.regs.general[0xF], 0x0);

        // draw the sprite in the corner, the rest of it shouldn't wrap
        chip8.regs.general[0xA] = 62;
        chip8.regs.general[0xB] = 30;
        execute_instruction(
            &Instruction::Drw {
                reg1: 0xA,
                reg2: 0xB,
                n_bytes: 3,
            },
            &mut chip8,
        );
        chip8.dbg_print_display();
        for x in 62..64 {
            for y in 30..32 {
                assert_eq!(chip8.get_px(x, y), 0x1);
            }
        }
        assert_eq!(chip8.regs.general[0xF], 0x0);
    }

    #[test]
    fn execute_skp_and_sknp_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.general[0xA] = 0x5;
        chip8.regs.general[0xB] = 0x6;
        chip8.key_down[0x6] = true;

        // SKP: no skip, key is not down
        execute_instruction(&Instruction::Skp { reg: 0xA }, &mut chip8);
        assert_eq!(chip8.regs.pc, 0x0);

        // SKP: skip, key is down
        execute_instruction(&Instruction::Skp { reg: 0xB }, &mut chip8);
        assert_eq!(chip8.regs.pc, 0x2);

        // SKNP: no skip, key is down
        execute_instruction(&Instruction::SkpNp { reg: 0xA }, &mut chip8);
        assert_eq!(chip8.regs.pc, 0x4);

        // SKNP: skip, key is down
        execute_instruction(&Instruction::SkpNp { reg: 0xB }, &mut chip8);
        assert_eq!(chip8.regs.pc, 0x4);
    }

    #[test]
    fn execute_ld_dt_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.dt = 0x12;

        // Ld from DT
        execute_instruction(&Instruction::LdFromDt { reg: 0xA }, &mut chip8);
        assert_eq!(chip8.regs.dt, 0x12);

        // Ld into DT
        execute_instruction(&Instruction::LdFromDt { reg: 0xA }, &mut chip8);
        assert_eq!(chip8.regs.dt, 0x12);
    }

    #[test]
    fn execute_ld_key_works() {
        let mut chip8 = Chip8::new();

        // key 5 was down
        chip8.key_down[5] = true;
        execute_instruction(&Instruction::LdKey { reg: 0xA }, &mut chip8);
        assert_eq!(chip8.regs.pc, 0x0); // no change to pc
        assert_eq!(chip8.regs.general[0xA], 0x5);

        // key 5 was down
        chip8.key_down[5] = false;
        chip8.regs.general[0xA] = 0;
        chip8.regs.pc += 2; // simulate pc increment
        execute_instruction(&Instruction::LdKey { reg: 0xA }, &mut chip8);
        assert_eq!(chip8.regs.pc, 0x0); // pc back 2 counts
        assert_eq!(chip8.regs.general[0xA], 0x0);
    }

    #[test]
    fn execute_ld_st_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.general[0xA] = 0x12;

        execute_instruction(&Instruction::LdSt { reg: 0xA }, &mut chip8);
        assert_eq!(chip8.regs.st, 0x12);
    }

    #[test]
    fn execute_add_i_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.i = 0x05;
        chip8.regs.general[0xA] = 0x12;

        execute_instruction(&Instruction::AddI { reg: 0xA }, &mut chip8);
        assert_eq!(chip8.regs.i, 0x17);
    }

    #[test]
    fn execute_ld_f_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.general[0xA] = 0x12;

        execute_instruction(&Instruction::LdF { reg: 0xA }, &mut chip8);
        assert_eq!(chip8.regs.i, 0x5A);
    }

    #[test]
    fn execute_ld_b_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.general[0xA] = 234;
        chip8.regs.i = 0x300;

        // breaks the decimal into digits and stores them at consecutive addresses
        execute_instruction(&Instruction::LdB { reg: 0xA }, &mut chip8);
        assert_eq!(chip8.memory[0x300], 2);
        assert_eq!(chip8.memory[0x301], 3);
        assert_eq!(chip8.memory[0x302], 4);
    }

    #[test]
    fn execute_ld_regs_mem_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.general[0x0] = 0xA;
        chip8.regs.general[0x1] = 0xB;
        chip8.regs.general[0x2] = 0xC;
        chip8.regs.general[0x3] = 0xD;
        chip8.regs.i = 0x300;

        execute_instruction(&Instruction::LdRegsMem { end_reg: 0x2 }, &mut chip8);
        assert_eq!(chip8.memory[0x300], 0xA);
        assert_eq!(chip8.memory[0x301], 0xB);
        assert_eq!(chip8.memory[0x302], 0xC);
        assert_eq!(chip8.memory[0x303], 0x0); // load only up to 0x2 reg
    }

    #[test]
    fn execute_ld_mem_regs_works() {
        let mut chip8 = Chip8::new();
        chip8.regs.i = 0x300;
        chip8.memory[0x300] = 0xA;
        chip8.memory[0x301] = 0xB;
        chip8.memory[0x302] = 0xC;
        chip8.memory[0x303] = 0xD;

        execute_instruction(&Instruction::LdMemRegs { end_reg: 0x2 }, &mut chip8);
        assert_eq!(chip8.regs.general[0x0], 0xA);
        assert_eq!(chip8.regs.general[0x1], 0xB);
        assert_eq!(chip8.regs.general[0x2], 0xC);
        assert_eq!(chip8.regs.general[0x3], 0x0);
    }
}
