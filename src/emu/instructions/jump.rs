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

pub fn call(cpu: &mut CPU, memory: &Memory) {
    cpu.sp = cpu.pc + 3;
    jp(cpu, memory);
}

pub fn ret_with_operand(cpu: &mut CPU, additional_operand: JpOperands) {
    let flag_value = match additional_operand {
        JpOperands::Z => cpu.registers.get_flag_z(),
        JpOperands::NZ => !cpu.registers.get_flag_z(),
        JpOperands::C => cpu.registers.get_flag_c(),
        JpOperands::NC => !cpu.registers.get_flag_c(),
    };
    if flag_value {
        ret(cpu);
    } else {
        cpu.pc += 1;
    }
}

pub fn ret(cpu: &mut CPU) {
    cpu.pc = cpu.sp;
}

pub fn reti(cpu: &mut CPU) {
    misc::ei(cpu);
    ret(cpu);
}

pub fn rst(cpu: &mut CPU, address: u8) {
    cpu.sp = cpu.pc + 1;
    cpu.pc = address as u16;
}
