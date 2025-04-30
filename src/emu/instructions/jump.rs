use crate::emu::cpu::CPU;
use crate::emu::memory::Memory;

use super::enums::JpOperands;
use super::utils;

pub fn jp_with_operand(cpu: &mut CPU, memory: &mut Memory, additional_operand: JpOperands) {
    let flag_value = match additional_operand {
        JpOperands::Z => cpu.registers.get_flag_z(),
        JpOperands::NZ => !cpu.registers.get_flag_z(),
        JpOperands::C => cpu.registers.get_flag_c(),
        JpOperands::NC => !cpu.registers.get_flag_c(),
    };
    if flag_value {
        jp(cpu, memory);
    } else {
        cpu.pc += 3;
    }
}

pub fn jp(cpu: &mut CPU, memory: &Memory) {
    cpu.pc = utils::get_next_bytes_little_endian(cpu, memory);
}

pub fn jp_hl(cpu: &mut CPU) {
    cpu.pc = cpu.registers.get_hl();
}
