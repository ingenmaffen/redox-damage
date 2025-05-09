use crate::emu::cpu::CPU;
use crate::emu::memory::Memory;

use super::enums::InstructionSourceTarget;
use super::utils;

pub fn and(cpu: &mut CPU, source: InstructionSourceTarget) {
    let and_value = match source {
        InstructionSourceTarget::B => cpu.registers.b,
        InstructionSourceTarget::C => cpu.registers.c,
        InstructionSourceTarget::D => cpu.registers.d,
        InstructionSourceTarget::E => cpu.registers.e,
        InstructionSourceTarget::H => cpu.registers.h,
        InstructionSourceTarget::L => cpu.registers.l,
        InstructionSourceTarget::HlAsPointer => panic!("This is handled in a separate function"),
        InstructionSourceTarget::A => cpu.registers.a,
        _ => panic!("Source not supported"),
    };
    cpu.registers.a = cpu.registers.a & and_value;
    set_and_flags(cpu);
    cpu.pc += 1;
}

pub fn and_hl(cpu: &mut CPU, memory: &Memory) {
    let and_value = utils::read_byte_from_memory(memory, cpu.registers.get_hl() as usize);
    cpu.registers.a = cpu.registers.a & and_value;
    set_and_flags(cpu);
    cpu.pc += 1;
}

pub fn and_n8(cpu: &mut CPU, memory: &Memory) {
    let and_value = utils::read_byte_from_memory(memory, cpu.pc as usize + 1);
    cpu.registers.a = cpu.registers.a & and_value;
    set_and_flags(cpu);
    cpu.pc += 2;
}

fn set_and_flags(cpu: &mut CPU) {
    // Flags: Z, 0, 1, 0
    cpu.registers.set_flag_z(cpu.registers.a == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(true);
    cpu.registers.set_flag_c(false);
}

pub fn xor(cpu: &mut CPU, source: InstructionSourceTarget) {
    let and_value = match source {
        InstructionSourceTarget::B => cpu.registers.b,
        InstructionSourceTarget::C => cpu.registers.c,
        InstructionSourceTarget::D => cpu.registers.d,
        InstructionSourceTarget::E => cpu.registers.e,
        InstructionSourceTarget::H => cpu.registers.h,
        InstructionSourceTarget::L => cpu.registers.l,
        InstructionSourceTarget::HlAsPointer => panic!("This is handled in a separate function"),
        InstructionSourceTarget::A => cpu.registers.a,
        _ => panic!("Source not supported"),
    };
    cpu.registers.a = cpu.registers.a ^ and_value;
    set_or_xor_flags(cpu);
    cpu.pc += 1;
}

pub fn xor_hl(cpu: &mut CPU, memory: &Memory) {
    let xor_value = utils::read_byte_from_memory(memory, cpu.registers.get_hl() as usize);
    cpu.registers.a = cpu.registers.a ^ xor_value;
    set_or_xor_flags(cpu);
    cpu.pc += 1;
}

pub fn xor_n8(cpu: &mut CPU, memory: &Memory) {
    let xor_value = utils::read_byte_from_memory(memory, cpu.pc as usize + 1);
    cpu.registers.a = cpu.registers.a ^ xor_value;
    set_or_xor_flags(cpu);
    cpu.pc += 2;
}

pub fn or(cpu: &mut CPU, source: InstructionSourceTarget) {
    let and_value = match source {
        InstructionSourceTarget::B => cpu.registers.b,
        InstructionSourceTarget::C => cpu.registers.c,
        InstructionSourceTarget::D => cpu.registers.d,
        InstructionSourceTarget::E => cpu.registers.e,
        InstructionSourceTarget::H => cpu.registers.h,
        InstructionSourceTarget::L => cpu.registers.l,
        InstructionSourceTarget::HlAsPointer => panic!("This is handled in a separate function"),
        InstructionSourceTarget::A => cpu.registers.a,
        _ => panic!("Source not supported"),
    };
    cpu.registers.a = cpu.registers.a | and_value;
    set_or_xor_flags(cpu);
    cpu.pc += 1;
}

pub fn or_hl(cpu: &mut CPU, memory: &Memory) {
    let or_value = utils::read_byte_from_memory(memory, cpu.registers.get_hl() as usize);
    cpu.registers.a = cpu.registers.a | or_value;
    set_or_xor_flags(cpu);
    cpu.pc += 1;
}

pub fn or_n8(cpu: &mut CPU, memory: &Memory) {
    let or_value = utils::read_byte_from_memory(memory, cpu.pc as usize + 1);
    cpu.registers.a = cpu.registers.a | or_value;
    set_or_xor_flags(cpu);
    cpu.pc += 2;
}

fn set_or_xor_flags(cpu: &mut CPU) {
    // Flags: Z, 0, 0, 0
    cpu.registers.set_flag_z(cpu.registers.a == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(false);
}
