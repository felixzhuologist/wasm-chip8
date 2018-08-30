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

    /// Reset the CPU and its display to their initial states
    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.cpu.load_rom(data);
    }

    /// Execute a single cycle of the CPU
    pub fn cycle(&mut self) {
        self.cpu.decrement_timers();
        let next_instruction = self.cpu.read_instruction();
        log!(
            "Processing instruction {:#X} at location {:?}",
            next_instruction,
            self.cpu.get_pc());
        self.cpu.process_instruction(next_instruction);
    }
}

