mod emu;

use emu::boot;
use emu::cpu::CPU;
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

    cpu.pc = 0x0100;
    cpu.sp = 0xFFFE;

    // TODO: rename
    let mut display = display::Display::default();
    // displaying a window is within an infinite loop
    // so the instruction execution has been moved there
    display.start_main_process(&mut cpu, &mut memory);
}
