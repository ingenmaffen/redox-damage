use crate::emu::cpu::CPU;
use crate::emu::memory::Memory;

pub fn get_next_bytes_little_endian(cpu: &CPU, memory: &Memory) -> u16 {
    let pc_as_usize: usize = cpu.pc as usize;
    let mut address: u16 = memory.addresses[pc_as_usize + 2] as u16;
    address = address << 8;
    address | memory.addresses[pc_as_usize + 1] as u16
}

pub fn get_e8(cpu: &CPU, memory: &Memory) -> i8 {
    memory.addresses[cpu.pc as usize + 1] as i8
}
