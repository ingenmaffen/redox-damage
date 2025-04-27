mod emu;

use emu::cpu::CPU;
use emu::memory::Memory;
use emu::rom::ROM;

fn main() {
    let mut cpu = CPU::default();
    let mut memory = Memory::default();
    let mut rom = ROM { data: None };
    rom.load_rom_from_file("rom.gb");
    if let Some(ref romdata) = rom.data {
        memory.load_rom_data_into_bank_00(romdata);
        memory.load_rom_data_into_bank_01(romdata);
    }
    // print_byte_vector(&memory.registers);
    cpu_register_debug(&mut cpu);
}

#[allow(dead_code)]
fn cpu_register_debug(cpu: &mut CPU) {
    println!("registers: {:#2X?}", cpu.registers);
    println!("AF with default values: {:#2X?}", cpu.registers.get_af());
    cpu.registers.set_af(0x5678);
    println!("registers: {:#2X?}", cpu.registers);
    println!("AF with updated values: {:#2X?}", cpu.registers.get_af());
    println!("CPU: {:#2X?}", cpu);
}

#[allow(dead_code)]
fn print_byte_vector(data: &Vec<u8>) {
    for byte in data {
        print!("{:2X} ", &byte)
    }
    println!("");
}
