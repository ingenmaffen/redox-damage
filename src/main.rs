mod emu;

use emu::cpu::CPU;
use emu::instructions;
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

    cpu.pc = 0x0100;
    while cpu.pc < 0xFFFF {
        instructions::execute_instruction(&mut cpu, &mut memory);
    }
}
