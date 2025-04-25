pub mod emu;

use emu::memory::Memory;
use emu::registers::Registers;
use emu::rom::ROM;

fn main() {
    // cpu_register_debug();
    let memory = Memory::default();
    // print!("{:?}", &memory);
    let mut rom = ROM { data: None };
    rom.load_rom_from_file("rom.gb");
    if let Some(romdata) = rom.data {
        print_byte_vector(romdata);
    }
}

#[allow(dead_code)]
fn cpu_register_debug() {
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
    println!("registers: {:#2X?}", registers);
}

#[allow(dead_code)]
fn print_byte_vector(data: Vec<u8>) {
    for byte in data {
        print!("{:2X} ", &byte)
    }
    println!("");
}
