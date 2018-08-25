use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::process;

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

    for i in (0..buffer.len()).step_by(2) {
        let ins: Instruction = ((buffer[i] as u16) << 8) | (buffer[i + 1] as u16);
        print_instruction(ins);
    }
}

fn print_instruction(instruction: Instruction) -> () {
    // separate out instruction nibbles
    let op1 = (instruction & 0xF000) >> 12;
    let op2 = (instruction & 0x0F00) >> 8;
    let op3 = (instruction & 0x00F0) >> 4;
    let op4 = instruction & 0x000F;

    // separate out the possible operands
    let nnn = instruction & 0x0FFF;
    let x = op2;
    let kk = instruction & 0x00FF;
    let y = op3;
    let n = op4;

    match (op1, op2, op3, op4) {
        (0, 0, 0xE, 0) => println!("CLS"),
        (0, 0, 0xE, 0xE) => println!("RET"),
        (0, _, _, _) => println!("Unsupported JUMP {:#x?}", instruction),
        (1, _, _, _) => println!("JP {:#x?}", nnn),
        (2, _, _, _) => println!("CALL {:#x?}", nnn),
        (3, _, _, _) => println!("SE {:#x?}, {:#x?}", x, kk),
        (4, _, _, _) => println!("SNE {:#x?}, {:#x?}", x, kk),
        (5, _, _, 0) => println!("SE {:#x?}, {:#x?}", x, y),
        (6, _, _, _) => println!("LD {:#x?}, {:#x?}", x, kk),
        (7, _, _, _) => println!("ADD {:#x?}, {:#x?}", x, kk),
        (8, _, _, 0) => println!("LD {:#x?}, {:#x?}", x, y),
        (8, _, _, 1) => println!("OR {:#x?}, {:#x?}", x, y),
        (8, _, _, 2) => println!("AND {:#x?}, {:#x?}", x, y),
        (8, _, _, 3) => println!("XOR {:#x?}, {:#x?}", x, y),
        (8, _, _, 4) => println!("ADD {:#x?}, {:#x?}", x, y),
        (8, _, _, 5) => println!("SUB {:#x?}, {:#x?}", x, y),
        (8, _, _, 6) => println!("SHR {:#x?}, {:#x?}", x, y),
        (8, _, _, 7) => println!("SUBN {:#x?}, {:#x?}", x, y),
        (8, _, _, 0xE) => println!("SHL {:#x?}, {:#x?}", x, y),
        (9, _, _, 0) => println!("SNE {:#x?}, {:#x?}", x, y),
        (0xA, _, _, _) => println!("LD {:#x?}", nnn),
        (0xB, _, _, _) => println!("JP {:#x?}", nnn),
        (0xC, _, _, _) => println!("RND {:#x?}, {:#x?}", x, kk),
        (0xD, _, _, _) => println!("DRW {:#x?}, {:#x?}, {:#x?}", x, y, n),
        (0xE, _, 9, 0xE) => println!("SKP {:#x?}", x),
        (0xE, _, 0xA, 1) => println!("SKNP {:#x?}", x),
        (0xF, _, 0, 7) => println!("LD {:#x?}", x),
        (0xF, _, 0, 0xA) => println!("LD {:#x?}", x),
        (0xF, _, 1, 5) => println!("LD {:#x?}", x),
        (0xF, _, 1, 8) => println!("LD {:#x?}", x),
        (0xF, _, 1, 0xE) => println!("ADD {:#x?}", x),
        (0xF, _, 2, 9) => println!("LD {:#x?}", x),
        (0xF, _, 3, 3) => println!("LD {:#x?}", x),
        (0xF, _, 5, 5) => println!("LD {:#x?}", x),
        (0xF, _, 6, 5) => println!("LD {:#x?}", x),
        _ => println!("Unknown instruction: {:#x?}", instruction)
    }
}