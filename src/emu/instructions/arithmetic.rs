use crate::emu::cpu::CPU;
use crate::emu::memory::Memory;

use super::enums::InstructionSourceTarget;

pub fn add(cpu: &mut CPU, source: InstructionSourceTarget) {
    let added_value = match source {
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
    set_add_flags(cpu, added_value);
    cpu.registers.a += added_value;
    cpu.pc += 1;
}

pub fn add_hl(cpu: &mut CPU, memory: &Memory) {
    let added_value: u8 = memory.addresses[cpu.registers.get_hl() as usize];
    set_add_flags(cpu, added_value);
    cpu.registers.a += added_value;
    cpu.pc += 1;
}

pub fn adc(cpu: &mut CPU, source: InstructionSourceTarget) {
    let mut added_value = match source {
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
    if cpu.registers.get_flag_c() {
        added_value += 1;
    }
    set_add_flags(cpu, added_value);
    cpu.registers.a += added_value;
    cpu.pc += 1;
}

pub fn adc_hl(cpu: &mut CPU, memory: &Memory) {
    let mut added_value: u8 = memory.addresses[cpu.registers.get_hl() as usize];
    if cpu.registers.get_flag_c() {
        added_value += 1;
    }
    set_add_flags(cpu, added_value);
    cpu.registers.a += added_value;
    cpu.pc += 1;
}

fn set_add_flags(cpu: &mut CPU, added_value: u8) {
    // Flags: Z, 0, H, C
    let new_value: u16 = cpu.registers.a as u16 + added_value as u16;
    cpu.registers.set_flag_z(new_value == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h((new_value & 0x0F) == 0);
    cpu.registers.set_flag_c(new_value > 0xFF);
}

pub fn sub(cpu: &mut CPU, source: InstructionSourceTarget) {
    let subtracted_value = match source {
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
    handle_sub(cpu, subtracted_value, false);
    cpu.pc += 1;
}

pub fn sub_hl(cpu: &mut CPU, memory: &Memory) {
    let subtracted_value: u8 = memory.addresses[cpu.registers.get_hl() as usize];
    handle_sub(cpu, subtracted_value, false);
    cpu.pc += 1;
}

pub fn cp(cpu: &mut CPU, source: InstructionSourceTarget) {
    let subtracted_value = match source {
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
    set_sub_flags(cpu, subtracted_value, false);
    cpu.pc += 1;
}

pub fn cp_hl(cpu: &mut CPU, memory: &Memory) {
    let subtracted_value: u8 = memory.addresses[cpu.registers.get_hl() as usize];
    set_sub_flags(cpu, subtracted_value, false);
    cpu.pc += 1;
}

pub fn sbc(cpu: &mut CPU, source: InstructionSourceTarget) {
    let mut subtracted_value = match source {
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
    if cpu.registers.get_flag_c() {
        subtracted_value += 1;
    }
    let subtracting_a: bool = matches!(source, InstructionSourceTarget::A);
    handle_sub(cpu, subtracted_value, subtracting_a);
    cpu.pc += 1;
}

pub fn sbc_hl(cpu: &mut CPU, memory: &Memory) {
    let mut subtracted_value: u8 = memory.addresses[cpu.registers.get_hl() as usize];
    if cpu.registers.get_flag_c() {
        subtracted_value += 1;
    }
    handle_sub(cpu, subtracted_value, false);
    cpu.pc += 1;
}

fn handle_sub(cpu: &mut CPU, subtracted_value: u8, is_sbc_a: bool) {
    set_sub_flags(cpu, subtracted_value, is_sbc_a);
    if cpu.registers.a >= subtracted_value {
        cpu.registers.a -= subtracted_value;
    } else {
        let diff: u8 = subtracted_value - cpu.registers.a;
        let new_a: u8 = 0xFF - diff + 1;
        cpu.registers.a += new_a;
    }
}

fn set_sub_flags(cpu: &mut CPU, subtracted_value: u8, is_sbc_a: bool) {
    // Flags: Z, 1, H, C
    let new_value: u8 = cpu.registers.a - subtracted_value;
    cpu.registers.set_flag_z(new_value == 0);
    cpu.registers.set_flag_n(true);
    cpu.registers.set_flag_h((new_value & 0xF0) == 0);
    if !is_sbc_a {
        cpu.registers.set_flag_c(cpu.registers.a < subtracted_value);
    }
}
