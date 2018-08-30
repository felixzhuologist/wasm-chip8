use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

#[wasm_bindgen]
pub struct CPUWrapper {
    cpu: ::cpu::CPU
}

#[wasm_bindgen]
impl CPUWrapper {
    /// Initialize a new CPU with undefined state. The user should call reset()
    /// on the new instance before using it
    pub fn new() -> CPUWrapper {
        CPUWrapper { cpu: ::cpu::CPU::new() }
    }

    /// Reset the CPU and its screen to their initial states
    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.cpu.load_rom(data);
    }

    /// Execute a single cycle of the CPU
    pub fn cycle(&mut self, debug: bool) {
        let next_instruction = self.cpu.read_instruction();
        if debug {
            for i in 0..16 {
                log!("V{}={}", i, self.cpu.v[i]);
            }
            log!("I={}", self.cpu.i);
            log!(
                "Processing instruction {:#X} at location {:?}",
                next_instruction,
                self.cpu.get_pc());
        }
        self.cpu.process_instruction(next_instruction);
    }

    pub fn decrement_timers(&mut self) {
        self.cpu.decrement_timers();
    }

    pub fn key_down(&mut self, key: u8) {
        self.cpu.keypad.key_down(key);
    }

    pub fn key_up(&mut self, key: u8) {
        self.cpu.keypad.key_up(key);
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.cpu.screen.get_pixel(x, y)
    }
}

