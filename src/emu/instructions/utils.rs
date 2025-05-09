use crate::emu::cpu::CPU;
use crate::emu::memory::Memory;

pub fn get_next_bytes_little_endian(cpu: &CPU, memory: &Memory) -> u16 {
    let pc_as_usize: usize = cpu.pc as usize;
    let mut address: u16 = read_byte_from_memory(memory, pc_as_usize + 2) as u16;
    address = address << 8;
    address | read_byte_from_memory(memory, pc_as_usize + 1) as u16
}

pub fn get_e8(cpu: &CPU, memory: &Memory) -> i8 {
    read_byte_from_memory(memory, cpu.pc as usize + 1) as i8
}

pub fn read_byte_from_memory(memory: &Memory, address: usize) -> u8 {
    if address >= 0xE000 && address < 0xFE00 {
        return memory.addresses[address - 0x2000];
    }
    return memory.addresses[address];
}

pub fn write_byte_to_memory(memory: &mut Memory, address: usize, value: u8) {
    memory.addresses[address] = value;
}
