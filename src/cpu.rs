use keypad::Keypad;
use screen::Screen;

const SPRITES: [u8; 80] = [
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
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct CPU {
    /// 4096 bytes of RAM. The first 512 bytes are where the original interpreter
    /// was located, so most programs start at location 512
    memory: [u8; 4096],
    /// 16 general purpose 8-bit registers `V0` through `VF`
    v: [u8; 16],
    /// 16-bit register used to index into memory
    i: u16,
    /// delay timer
    delay: u8,
    /// sound timer
    sound: u8,
    /// the program counter is an index into RAM pointing to the start of
    /// the next instruction. since instructions are 16 bits, the pc should
    /// always be even (otherwise it would be pointing to the middle of an
    /// instruction)
    pc: u16,
    /// stack pointer
    sp: u8,
    /// call stack
    stack: [u16; 16],
    /// 16 key keypad
    keypad: Keypad,
    /// 64x32 pixel monochrome display
    screen: Screen
}

impl CPU {
    /// Initialize a new CPU with undefined state. The user should call reset()
    /// on the new instance before using it
    pub fn new() -> CPU {
        CPU {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            delay: 0,
            sound: 0,
            pc: 0,
            sp: 0,
            stack: [0; 16],
            keypad: Keypad::new(),
            screen: Screen::new()
        }
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    /// Reset the CPU and its display to their initial states
    pub fn reset(&mut self) {
        for i in 0..4096 {
            self.memory[i] = 0;
        }
        for i in 0..80 {
            self.memory[i] = SPRITES[i];
        }
        for i in 0..16 {
            self.v[i] = 0;
            self.stack[i] = 0;
        }
        self.i = 0;
        self.screen.clear();
        self.delay = 255;
        self.sound = 255;
        self.pc = 512;
        self.sp = 0;
    }

    pub fn decrement_timers(&mut self) {
       if self.delay > 0 {
        self.delay -= 1;
       } 
       if self.sound > 0 {
        self.sound -= 1;
       }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        for i in 0..data.len() {
            self.memory[512 + i] = data[i];
        }
    }

    /// Read a single instruction at the program counter in memory
    pub fn read_instruction(&self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 |
        (self.memory[(self.pc + 1) as usize] as u16)
    }

    /// Process a single instruction and update the state of the CPU.
    /// This method is responsible for incrementing the PC after
    /// exeucting the instruction but does not decrement the delay timers
    pub fn process_instruction(&mut self, instruction: u16) -> () {
        // separate out instruction nibbles
        let op1 = (instruction & 0xF000) >> 12;
        let op2 = (instruction & 0x0F00) >> 8;
        let op3 = (instruction & 0x00F0) >> 4;
        let op4 = instruction & 0x000F;

        // separate out the possible operands
        let nnn = instruction & 0x0FFF;
        let kk = (instruction & 0x00FF) as u8;
        let x = op2 as usize;
        let y = op3 as usize;
        let n = op4 as u8;

        self.pc += 2;
        match (op1, op2, op3, op4) {
            (0, 0, 0xE, 0) => self.screen.clear(),
            (0, 0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            },
            (0, ..) => println!("Unsupported subroutine {:#x?}", instruction),
            (1, ..) => self.pc = nnn,
            (2, ..) => {
                // TODO: check for stack overflows
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            (3, ..) => self.pc += if self.v[x] == kk { 2 } else { 0 },
            (4, ..) => self.pc += if self.v[x] != kk { 2 } else { 0 },
            (5, .., 0) => self.pc += if self.v[x] == self.v[y] { 2 } else { 0 },
            (6, ..) => self.v[x] = kk as u8,
            (7, ..) => self.v[x] = self.v[x].wrapping_add(kk),
            (8, .., 0) => self.v[x] = self.v[y],
            (8, .., 1) => self.v[x] |= self.v[y],
            (8, .., 2) => self.v[x] &= self.v[y],
            (8, .., 3) => self.v[x] ^= self.v[y],
            (8, .., 4) => {
                let (sum, carry) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = sum;
                self.v[0xF] = carry as u8;
            },
            (8, .., 5) => {
                let (diff, borrow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = diff;
                self.v[0xF] = (!borrow) as u8;
            },
            (8, .., 6) => {
                self.v[0xF] = self.v[y] & 1;
                self.v[x] = self.v[y] >> 1;
            }
            (8, .., 7) => {
                let (diff, borrow) = self.v[y].overflowing_sub(self.v[x]);
                self.v[x] = diff;
                self.v[0xF] = (!borrow) as u8;
            },
            (8, .., 0xE) => {
                self.v[0xF] = self.v[y] >> 7;
                self.v[x] = self.v[y] << 1;
            },
            (9, .., 0) => self.pc += if self.v[x] != self.v[y] { 2 } else { 0 },
            (0xA, ..) => self.i = nnn,
            (0xB, ..) => self.pc = nnn + (self.v[0] as u16),
            // TODO: randomize
            (0xC, ..) => self.v[x] = 0xFF & kk,
            (0xD, ..) => {
                let (start, end) = (self.i as usize, (self.i + (n as u16)) as usize);
                self.v[0xF] = self.screen.draw_sprite(x, y, &self.memory[start .. end]) as u8;
            },
            (0xE, _, 9, 0xE) => {
                if self.keypad.is_key_down(self.v[x]) {
                    self.pc += 2
                }
            },
            (0xE, _, 0xA, 1) => {
                if !self.keypad.is_key_down(self.v[x]) {
                    self.pc += 2
                }
            },
            (0xF, _, 0, 7) => self.v[x] = self.delay,
            (0xF, _, 0, 0xA) => self.v[x] = self.keypad.wait_for_key_down(),
            (0xF, _, 1, 5) => self.delay = self.v[x],
            (0xF, _, 1, 8) => self.sound = self.v[x],
            (0xF, _, 1, 0xE) => self.i += self.v[x] as u16,
            // TODO check that 0 <= vX <= 16
            (0xF, _, 2, 9) => self.i = self.v[x] as u16 * 5,
            (0xF, _, 3, 3) => {
                // TODO: check bounds on i register
                self.memory[self.i as usize] = self.v[x] % 10;
                self.memory[(self.i + 1) as usize] = (self.v[x] / 10) % 10;
                self.memory[(self.i + 2) as usize] = (self.v[x] / 100) % 10;
            },
            (0xF, _, 5, 5) => {
                for i in 0..x {
                    self.memory[self.i as usize] = self.v[i];
                    self.i += 1;
                }
            },
            (0xF, _, 6, 5) => {
                for i in 0..x {
                    self.v[i] = self.memory[self.i as usize];
                    self.i += 1;
                }
            },
            _ => println!("Unknown instruction: {:#x?}", instruction)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn store_const_in_reg() {
        let mut cpu = CPU::new();
        cpu.process_instruction(0x63FE);
        assert_eq!(cpu.v[3], 0xFE);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn store_reg_in_reg() {
        let mut cpu = CPU::new();
        cpu.v[5] = 0xFE;
        cpu.process_instruction(0x8250);
        assert_eq!(cpu.v[2], 0xFE);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn add_const_to_reg() {
        let mut cpu = CPU::new();
        cpu.v[4] = 0xFF;
        cpu.process_instruction(0x7403);
        assert_eq!(cpu.v[4], 2);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn add_reg_to_reg() {
        let mut cpu = CPU::new();
        cpu.v[0xF] = 1;
        cpu.v[1] = 17;
        cpu.v[2] = 13;
        cpu.process_instruction(0x8124);
        assert_eq!(cpu.v[1], 30);
        assert_eq!(cpu.v[0xF], 0);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn add_reg_to_reg_overflow() {
        let mut cpu = CPU::new();
        cpu.v[0xF] = 0;
        cpu.v[1] = 255;
        cpu.v[2] = 13;
        cpu.process_instruction(0x8124);
        assert_eq!(cpu.v[1], 12);
        assert_eq!(cpu.v[0xF], 1);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn sub_reg_from_reg() {
        let mut cpu = CPU::new();
        cpu.v[0xF] = 1;
        cpu.v[1] = 200;
        cpu.v[2] = 10;
        cpu.process_instruction(0x8125);
        assert_eq!(cpu.v[1], 190);
        assert_eq!(cpu.v[0xF], 1);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn sub_reg_from_reg_rev_overflow() {
        let mut cpu = CPU::new();
        cpu.v[0xF] = 0;
        cpu.v[1] = 6;
        cpu.v[2] = 0;
        cpu.process_instruction(0x8127);
        assert_eq!(cpu.v[1], 250);
        assert_eq!(cpu.v[0xF], 0);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn reg_and() {
        let mut cpu = CPU::new();
        cpu.v[1] = 31;
        cpu.v[2] = 0;
        cpu.process_instruction(0x8122);
        assert_eq!(cpu.v[1], 0);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn reg_or() {
        let mut cpu = CPU::new();
        cpu.v[1] = 0;
        cpu.v[2] = 31;
        cpu.process_instruction(0x8121);
        assert_eq!(cpu.v[1], 31);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn reg_xor() {
        let mut cpu = CPU::new();
        cpu.v[1] = 3;
        cpu.v[2] = 1;
        cpu.process_instruction(0x8123);
        assert_eq!(cpu.v[1], 3 ^ 1);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn res_shift_right() {
        let mut cpu = CPU::new();
        cpu.v[10] = 0b01001101;
        cpu.process_instruction(0x80A6);
        assert_eq!(cpu.v[0], 0b00100110);
        assert_eq!(cpu.v[0xF], 1);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn res_shift_left() {
        let mut cpu = CPU::new();
        cpu.v[0xF] = 1;
        cpu.v[10] = 0b01001101;
        cpu.process_instruction(0x81AE);
        assert_eq!(cpu.v[1], 0b10011010);
        assert_eq!(cpu.v[0xF], 0);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn jump() {
        let mut cpu = CPU::new();
        cpu.process_instruction(0x1123);
        assert_eq!(cpu.pc, 0x0123);
    }

    #[test]
    fn jump_offset() {
        let mut cpu = CPU::new();
        cpu.v[0] = 10;
        cpu.process_instruction(0xB005);
        assert_eq!(cpu.pc, 15);
    }

    #[test]
    fn func_call() {
        let mut cpu = CPU::new();
        cpu.pc = 100;
        cpu.process_instruction(0x2123);
        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0x0123);

        cpu.process_instruction(0x00EE);
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.pc, 102);
    }

    #[test]
    fn skip_eq() {
        let mut cpu = CPU::new();
        cpu.v[5] = 5;
        cpu.process_instruction(0x3505);
        assert_eq!(cpu.pc, 4);

        cpu.process_instruction(0x3506);
        assert_eq!(cpu.pc, 6);
    }

    #[test]
    fn skip_eq_reg() {
        let mut cpu = CPU::new();
        cpu.v[4] = 4;
        cpu.v[5] = 4;
        cpu.process_instruction(0x5450);
        assert_eq!(cpu.pc, 4);

        cpu.v[5] = 5;
        cpu.process_instruction(0x5450);
        assert_eq!(cpu.pc, 6);
    }

    #[test]
    fn skip_neq() {
        let mut cpu = CPU::new();
        cpu.v[2] = 10;
        cpu.process_instruction(0x420A);
        assert_eq!(cpu.pc, 2);

        cpu.process_instruction(0x4200);
        assert_eq!(cpu.pc, 6);
    }

    #[test]
    fn skip_neq_reg() {
        let mut cpu = CPU::new();
        cpu.v[0xA] = 4;
        cpu.v[0xC] = 4;
        cpu.process_instruction(0x9CA0);
        assert_eq!(cpu.pc, 2);

        cpu.v[0xC] = 10;
        cpu.process_instruction(0x9CA0);
        assert_eq!(cpu.pc, 6);
    }

    #[test]
    fn get_set_delay() {
        let mut cpu = CPU::new();
        cpu.v[0xB] = 17;
        cpu.process_instruction(0xFB15);
        assert_eq!(cpu.delay, 17);

        cpu.process_instruction(0xFA07);
        assert_eq!(cpu.v[0xA], 17);
    }

    #[test]
    fn set_sound() {
        let mut cpu = CPU::new();
        cpu.v[0xE] = 211;
        cpu.process_instruction(0xFE18);
        assert_eq!(cpu.sound, 211);
    }

    #[test]
    fn set_i() {
        let mut cpu = CPU::new();
        cpu.process_instruction(0xA0FB);
        assert_eq!(cpu.i, 0xFB);
    }

    #[test]
    fn incr_i() {
        let mut cpu = CPU::new();
        cpu.v[3] = 20;
        cpu.i = 3012;
        cpu.process_instruction(0xF31E);
        assert_eq!(cpu.i, 3032);
    }
}
