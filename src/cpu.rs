use keypad::Keypad;
use screen::Screen;

pub struct CPU {
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub i: u16,
    pub delay: u8,
    pub sound: u8,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
    pub keypad: Keypad,
    pub screen: Screen
}

impl CPU {
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

        match (op1, op2, op3, op4) {
            (0, 0, 0xE, 0) => self.screen.clear(),
            (0, 0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            },
            (0, _, _, _) => println!("Unsupported subroutine {:#x?}", instruction),
            (1, _, _, _) => self.pc = nnn,
            (2, _, _, _) => {
                // TODO: check for stack overflows
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            (3, _, _, _) => {
                if self.v[x] == kk {
                    self.pc += 1;
                }
            },
            (4, _, _, _) => {
                if self.v[x] != kk {
                    self.pc += 1;
                }
            },
            (5, _, _, 0) => {
                if self.v[x] == self.v[y] {
                    self.pc += 1;
                }
            },
            (6, _, _, _) => self.v[x] = kk as u8,
            (7, _, _, _) => self.v[x] = self.v[x].wrapping_add(kk),
            (8, _, _, 0) => self.v[x] = self.v[y],
            (8, _, _, 1) => self.v[x] |= self.v[y],
            (8, _, _, 2) => self.v[x] &= self.v[y],
            (8, _, _, 3) => self.v[x] ^= self.v[y],
            (8, _, _, 4) => {
                let (sum, carry) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = sum;
                self.v[0xF] = carry as u8;
            },
            (8, _, _, 5) => {
                let (diff, borrow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = diff;
                self.v[0xF] = (!borrow) as u8;
            },
            (8, _, _, 6) => {
                self.v[0xF] = self.v[y] & 1;
                self.v[x] = self.v[y] >> 1;
            }
            (8, _, _, 7) => {
                let (diff, borrow) = self.v[y].overflowing_sub(self.v[x]);
                self.v[x] = diff;
                self.v[0xF] = (!borrow) as u8;
            },
            (8, _, _, 0xE) => {
                self.v[0xF] = self.v[y] >> 7;
                self.v[x] = self.v[y] << 1;
            },
            (9, _, _, 0) => {
                if self.v[x] != self.v[y] {
                    self.pc += 1;
                }
            },
            (0xA, _, _, _) => self.i = nnn,
            (0xB, _, _, _) => self.pc = nnn + (self.v[0] as u16),
            // TODO: randomize
            (0xC, _, _, _) => self.v[x] = 0xFF & kk,
            (0xD, _, _, _) => {
                let (start, end) = (self.i as usize, (self.i + (n as u16)) as usize);
                self.v[0xF] = self.screen.draw_sprite(x, y, &self.memory[start .. end]) as u8;
            },
            (0xE, _, 9, 0xE) => {
                if self.keypad.is_key_down(self.v[x]) {
                    self.pc += 1
                }
            },
            (0xE, _, 0xA, 1) => {
                if !self.keypad.is_key_down(self.v[x]) {
                    self.pc += 1
                }
            },
            (0xF, _, 0, 7) => self.v[x] = self.delay,
            // TODO: wait for a keypress and store the result in register VX
            (0xF, _, 0, 0xA) => println!("LD {:#x?}", x),
            (0xF, _, 1, 5) => self.delay = self.v[x],
            (0xF, _, 1, 8) => self.sound = self.v[x],
            (0xF, _, 1, 0xE) => self.i = self.v[x] as u16,
            (0xF, _, 2, 9) => println!("LD {:#x?}", x),
            (0xF, _, 3, 3) => {
                // TODO: check bounds on i register
                self.memory[self.i as usize] = self.v[x] % 10;
                self.memory[(self.i + 1) as usize] = (self.v[x] / 10) % 10;
                self.memory[(self.i + 2) as usize] = (self.v[x] / 100) % 10;
            },
            (0xF, _, 5, 5) => {
                for i in (0..x) {
                    self.memory[self.i as usize] = self.v[i];
                    self.i += 1;
                }
            },
            (0xF, _, 6, 5) => {
                for i in (0..x) {
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
    }

    #[test]
    fn store_reg_in_reg() {
        let mut cpu = CPU::new();
        cpu.v[5] = 0xFE;
        cpu.process_instruction(0x8250);
        assert_eq!(cpu.v[2], 0xFE);
    }

    #[test]
    fn add_const_to_reg() {
        let mut cpu = CPU::new();
        cpu.v[4] = 0xFF;
        cpu.process_instruction(0x7403);
        assert_eq!(cpu.v[4], 2);
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
    }

    #[test]
    fn reg_and() {
        let mut cpu = CPU::new();
        cpu.v[1] = 31;
        cpu.v[2] = 0;
        cpu.process_instruction(0x8122);
        assert_eq!(cpu.v[1], 0);
    }

    #[test]
    fn reg_or() {
        let mut cpu = CPU::new();
        cpu.v[1] = 0;
        cpu.v[2] = 31;
        cpu.process_instruction(0x8121);
        assert_eq!(cpu.v[1], 31);
    }

    #[test]
    fn reg_xor() {
        let mut cpu = CPU::new();
        cpu.v[1] = 3;
        cpu.v[2] = 1;
        cpu.process_instruction(0x8123);
        assert_eq!(cpu.v[1], 3 ^ 1);
    }

    #[test]
    fn res_shift_right() {
        let mut cpu = CPU::new();
        cpu.v[10] = 0b01001101;
        cpu.process_instruction(0x80A6);
        assert_eq!(cpu.v[0], 0b00100110);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn res_shift_left() {
        let mut cpu = CPU::new();
        cpu.v[0xF] = 1;
        cpu.v[10] = 0b01001101;
        cpu.process_instruction(0x81AE);
        assert_eq!(cpu.v[1], 0b10011010);
        assert_eq!(cpu.v[0xF], 0);
    }
}
