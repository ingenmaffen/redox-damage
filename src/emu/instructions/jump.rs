use crate::emu::cpu::CPU;
use crate::emu::memory::Memory;

use super::enums::JpOperands;
use super::misc;
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

pub fn jr_with_operand(cpu: &mut CPU, memory: &mut Memory, additional_operand: JpOperands) {
    let flag_value = match additional_operand {
        JpOperands::Z => cpu.registers.get_flag_z(),
        JpOperands::NZ => !cpu.registers.get_flag_z(),
        JpOperands::C => cpu.registers.get_flag_c(),
        JpOperands::NC => !cpu.registers.get_flag_c(),
    };
    if flag_value {
        jr(cpu, memory);
    } else {
        cpu.pc += 2;
    }
}

pub fn jr(cpu: &mut CPU, memory: &Memory) {
    let relative_address: i8 = utils::get_e8(cpu, memory);
    cpu.pc += 2;
    if relative_address < 0 {
        cpu.pc -= relative_address.abs() as u16;
    } else {
        cpu.pc += relative_address as u16;
    }
}

pub fn call_with_operand(cpu: &mut CPU, memory: &mut Memory, additional_operand: JpOperands) {
    let flag_value = match additional_operand {
        JpOperands::Z => cpu.registers.get_flag_z(),
        JpOperands::NZ => !cpu.registers.get_flag_z(),
        JpOperands::C => cpu.registers.get_flag_c(),
        JpOperands::NC => !cpu.registers.get_flag_c(),
    };
    if flag_value {
        call(cpu, memory);
    } else {
        cpu.pc += 3;
    }
}

pub fn call(cpu: &mut CPU, memory: &mut Memory) {
    cpu.pc += 3;
    let value1: u8 = (cpu.pc >> 8) as u8;
    let value2: u8 = (cpu.pc & 0x00FF) as u8;
    memory.addresses[cpu.sp as usize] = value1;
    memory.addresses[cpu.sp as usize - 1] = value2;
    cpu.sp -= 2;
    jp(cpu, memory);
}

pub fn ret_with_operand(cpu: &mut CPU, memory: &Memory, additional_operand: JpOperands) {
    let flag_value = match additional_operand {
        JpOperands::Z => cpu.registers.get_flag_z(),
        JpOperands::NZ => !cpu.registers.get_flag_z(),
        JpOperands::C => cpu.registers.get_flag_c(),
        JpOperands::NC => !cpu.registers.get_flag_c(),
    };
    if flag_value {
        ret(cpu, memory);
    } else {
        cpu.pc += 1;
    }
}

pub fn ret(cpu: &mut CPU, memory: &Memory) {
    let value1: u16 = memory.addresses[cpu.sp as usize] as u16;
    let value2: u16 = memory.addresses[cpu.sp as usize + 1] as u16;
    let value: u16 = value2 << 8 | value1;
    cpu.pc = value;
    cpu.sp += 2;
}

pub fn reti(cpu: &mut CPU, memory: &Memory) {
    misc::ei(cpu);
    ret(cpu, memory);
}

pub fn rst(cpu: &mut CPU, address: u8) {
    cpu.sp = cpu.pc + 1;
    cpu.pc = address as u16;
}
