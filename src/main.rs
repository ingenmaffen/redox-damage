mod emu;

use emu::boot;
use emu::cpu::CPU;
use emu::instruction_mapper;
use emu::io::display;
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

    boot::boot_sequence(&mut memory);

    let mut display = display::Display::default();
    display.construct_vram_content(&memory);
    display.render();

    cpu.pc = 0x0100;
    cpu.sp = 0xFFFE;

    // while cpu.pc < 0xFFFF {
    instruction_mapper::execute_instruction(&mut cpu, &mut memory);
    // }
}
