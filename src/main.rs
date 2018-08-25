extern crate chip8;

use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::process;

use chip8::cpu::CPU;

type Instruction = u16;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut f = File::open(&args[1]).expect("file not found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("could not read");
    println!("size of buffer: {}", buffer.len());

    if buffer.len() % 2 == 1 {
        eprintln!("Incomplete instructions in the rom, aborting");
        process::exit(1);
    }

    let mut cpu = CPU::new();
    for i in (0..buffer.len()).step_by(2) {
        let ins: Instruction = ((buffer[i] as u16) << 8) | (buffer[i + 1] as u16);
        cpu.process_instruction(ins);
    }
}