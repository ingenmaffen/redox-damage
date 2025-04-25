use std::fs;

use crate::emu::registers::Registers;

pub mod emu;

fn main() {
    let mut registers = Registers {
        a: 0x12,
        f: 0x34,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        h: 0,
        l: 0,
    };

    println!("registers: {:#2X?}", registers);
    println!("AF as 1234: {:#2X?}", registers.get_af());
    registers.set_af(0x5678);
    println!("A: {:#2X}, F: {:#2X}", registers.a, registers.f);
    // load_rom();
}

#[allow(dead_code)]
fn load_rom() {
    let contents = fs::read("rom.gb").expect("Should have been able to read the file");

    for byte in contents {
        print!("{:2X} ", &byte)
    }
    println!("");
}
