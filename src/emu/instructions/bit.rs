use crate::emu::cpu::CPU;
use crate::emu::memory::Memory;

use super::enums::InstructionSourceTarget;

pub fn rlc(cpu: &mut CPU, target: InstructionSourceTarget) {
    match target {
        InstructionSourceTarget::B => cpu.registers.b = get_rlc_result_and_set_flags(cpu, cpu.registers.b),
        InstructionSourceTarget::C => cpu.registers.c = get_rlc_result_and_set_flags(cpu, cpu.registers.c),
        InstructionSourceTarget::D => cpu.registers.d = get_rlc_result_and_set_flags(cpu, cpu.registers.d),
        InstructionSourceTarget::E => cpu.registers.e = get_rlc_result_and_set_flags(cpu, cpu.registers.e),
        InstructionSourceTarget::H => cpu.registers.h = get_rlc_result_and_set_flags(cpu, cpu.registers.h),
        InstructionSourceTarget::L => cpu.registers.l = get_rlc_result_and_set_flags(cpu, cpu.registers.l),
        InstructionSourceTarget::HlAsPointer => panic!("This is handled in a separate function"),
        InstructionSourceTarget::A => cpu.registers.a = get_rlc_result_and_set_flags(cpu, cpu.registers.a),
        _ => panic!("Source not supported"),
    };
}

pub fn rlc_hl(cpu: &mut CPU, memory: &mut Memory) {
    let address: usize = cpu.registers.get_hl() as usize;
    memory.addresses[address] = get_rlc_result_and_set_flags(cpu, memory.addresses[address]);
}

fn get_rlc_result_and_set_flags(cpu: &mut CPU, original_value: u8) -> u8 {
    let carry: bool = (original_value & 0b10000000) > 0;
    let mut result: u8 = original_value << 1;
    if carry {
        result = result | 0b00000001;
    }
    set_rotate_shift_flags(cpu, result, carry);
    result
}

fn set_rotate_shift_flags(cpu: &mut CPU, result: u8, carry: bool) {
    // Flags: Z, 0, 0, C
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(carry);
}

pub fn rrc(cpu: &mut CPU, target: InstructionSourceTarget) {
    match target {
        InstructionSourceTarget::B => cpu.registers.b = get_rrc_result_and_set_flags(cpu, cpu.registers.b),
        InstructionSourceTarget::C => cpu.registers.c = get_rrc_result_and_set_flags(cpu, cpu.registers.c),
        InstructionSourceTarget::D => cpu.registers.d = get_rrc_result_and_set_flags(cpu, cpu.registers.d),
        InstructionSourceTarget::E => cpu.registers.e = get_rrc_result_and_set_flags(cpu, cpu.registers.e),
        InstructionSourceTarget::H => cpu.registers.h = get_rrc_result_and_set_flags(cpu, cpu.registers.h),
        InstructionSourceTarget::L => cpu.registers.l = get_rrc_result_and_set_flags(cpu, cpu.registers.l),
        InstructionSourceTarget::HlAsPointer => panic!("This is handled in a separate function"),
        InstructionSourceTarget::A => cpu.registers.a = get_rrc_result_and_set_flags(cpu, cpu.registers.a),
        _ => panic!("Source not supported"),
    };
}

pub fn rrc_hl(cpu: &mut CPU, memory: &mut Memory) {
    let address: usize = cpu.registers.get_hl() as usize;
    memory.addresses[address] = get_rrc_result_and_set_flags(cpu, memory.addresses[address]);
}

fn get_rrc_result_and_set_flags(cpu: &mut CPU, original_value: u8) -> u8 {
    let carry: bool = (original_value & 0b00000001) > 0;
    let mut result: u8 = original_value >> 1;
    if carry {
        result = result | 0b10000000;
    }
    set_rotate_shift_flags(cpu, result, carry);
    result
}

pub fn rl(cpu: &mut CPU, target: InstructionSourceTarget) {
    match target {
        InstructionSourceTarget::B => cpu.registers.b = get_rl_result_and_set_flags(cpu, cpu.registers.b),
        InstructionSourceTarget::C => cpu.registers.c = get_rl_result_and_set_flags(cpu, cpu.registers.c),
        InstructionSourceTarget::D => cpu.registers.d = get_rl_result_and_set_flags(cpu, cpu.registers.d),
        InstructionSourceTarget::E => cpu.registers.e = get_rl_result_and_set_flags(cpu, cpu.registers.e),
        InstructionSourceTarget::H => cpu.registers.h = get_rl_result_and_set_flags(cpu, cpu.registers.h),
        InstructionSourceTarget::L => cpu.registers.l = get_rl_result_and_set_flags(cpu, cpu.registers.l),
        InstructionSourceTarget::HlAsPointer => panic!("This is handled in a separate function"),
        InstructionSourceTarget::A => cpu.registers.a = get_rl_result_and_set_flags(cpu, cpu.registers.a),
        _ => panic!("Source not supported"),
    };
}

pub fn rl_hl(cpu: &mut CPU, memory: &mut Memory) {
    let address: usize = cpu.registers.get_hl() as usize;
    memory.addresses[address] = get_rl_result_and_set_flags(cpu, memory.addresses[address]);
}

fn get_rl_result_and_set_flags(cpu: &mut CPU, original_value: u8) -> u8 {
    let carry: bool = (original_value & 0b10000000) > 0;
    let mut result: u8 = original_value << 1;
    if cpu.registers.get_flag_c() {
        result = result | 0b00000001;
    }
    set_rotate_shift_flags(cpu, result, carry);
    result
}

pub fn rr(cpu: &mut CPU, target: InstructionSourceTarget) {
    match target {
        InstructionSourceTarget::B => cpu.registers.b = get_rr_result_and_set_flags(cpu, cpu.registers.b),
        InstructionSourceTarget::C => cpu.registers.c = get_rr_result_and_set_flags(cpu, cpu.registers.c),
        InstructionSourceTarget::D => cpu.registers.d = get_rr_result_and_set_flags(cpu, cpu.registers.d),
        InstructionSourceTarget::E => cpu.registers.e = get_rr_result_and_set_flags(cpu, cpu.registers.e),
        InstructionSourceTarget::H => cpu.registers.h = get_rr_result_and_set_flags(cpu, cpu.registers.h),
        InstructionSourceTarget::L => cpu.registers.l = get_rr_result_and_set_flags(cpu, cpu.registers.l),
        InstructionSourceTarget::HlAsPointer => panic!("This is handled in a separate function"),
        InstructionSourceTarget::A => cpu.registers.a = get_rr_result_and_set_flags(cpu, cpu.registers.a),
        _ => panic!("Source not supported"),
    };
}

pub fn rr_hl(cpu: &mut CPU, memory: &mut Memory) {
    let address: usize = cpu.registers.get_hl() as usize;
    memory.addresses[address] = get_rr_result_and_set_flags(cpu, memory.addresses[address]);
}

fn get_rr_result_and_set_flags(cpu: &mut CPU, original_value: u8) -> u8 {
    let carry: bool = (original_value & 0b00000001) > 0;
    let mut result: u8 = original_value >> 1;
    if cpu.registers.get_flag_c() {
        result = result | 0b10000000;
    }
    set_rotate_shift_flags(cpu, result, carry);
    result
}

pub fn sla(cpu: &mut CPU, target: InstructionSourceTarget) {
    match target {
        InstructionSourceTarget::B => cpu.registers.b = get_sla_result_and_set_flags(cpu, cpu.registers.b),
        InstructionSourceTarget::C => cpu.registers.c = get_sla_result_and_set_flags(cpu, cpu.registers.c),
        InstructionSourceTarget::D => cpu.registers.d = get_sla_result_and_set_flags(cpu, cpu.registers.d),
        InstructionSourceTarget::E => cpu.registers.e = get_sla_result_and_set_flags(cpu, cpu.registers.e),
        InstructionSourceTarget::H => cpu.registers.h = get_sla_result_and_set_flags(cpu, cpu.registers.h),
        InstructionSourceTarget::L => cpu.registers.l = get_sla_result_and_set_flags(cpu, cpu.registers.l),
        InstructionSourceTarget::HlAsPointer => panic!("This is handled in a separate function"),
        InstructionSourceTarget::A => cpu.registers.a = get_sla_result_and_set_flags(cpu, cpu.registers.a),
        _ => panic!("Source not supported"),
    };
}

pub fn sla_hl(cpu: &mut CPU, memory: &mut Memory) {
    let address: usize = cpu.registers.get_hl() as usize;
    memory.addresses[address] = get_sla_result_and_set_flags(cpu, memory.addresses[address]);
}

fn get_sla_result_and_set_flags(cpu: &mut CPU, original_value: u8) -> u8 {
    let carry: bool = (original_value & 0b10000000) > 0;
    let result: u8 = original_value << 1;
    set_rotate_shift_flags(cpu, result, carry);
    result
}

pub fn sra(cpu: &mut CPU, target: InstructionSourceTarget) {
    match target {
        InstructionSourceTarget::B => cpu.registers.b = get_sra_result_and_set_flags(cpu, cpu.registers.b),
        InstructionSourceTarget::C => cpu.registers.c = get_sra_result_and_set_flags(cpu, cpu.registers.c),
        InstructionSourceTarget::D => cpu.registers.d = get_sra_result_and_set_flags(cpu, cpu.registers.d),
        InstructionSourceTarget::E => cpu.registers.e = get_sra_result_and_set_flags(cpu, cpu.registers.e),
        InstructionSourceTarget::H => cpu.registers.h = get_sra_result_and_set_flags(cpu, cpu.registers.h),
        InstructionSourceTarget::L => cpu.registers.l = get_sra_result_and_set_flags(cpu, cpu.registers.l),
        InstructionSourceTarget::HlAsPointer => panic!("This is handled in a separate function"),
        InstructionSourceTarget::A => cpu.registers.a = get_sra_result_and_set_flags(cpu, cpu.registers.a),
        _ => panic!("Source not supported"),
    };
}

pub fn sra_hl(cpu: &mut CPU, memory: &mut Memory) {
    let address: usize = cpu.registers.get_hl() as usize;
    memory.addresses[address] = get_sra_result_and_set_flags(cpu, memory.addresses[address]);
}

fn get_sra_result_and_set_flags(cpu: &mut CPU, original_value: u8) -> u8 {
    let original_upper_bit: bool = original_value & 0b10000000 > 0;
    let carry: bool = (original_value & 0b00000001) > 0;
    let mut result: u8 = original_value >> 1;
    if original_upper_bit {
        result = result | 0b10000000;
    }
    set_rotate_shift_flags(cpu, result, carry);
    result
}

pub fn swap(cpu: &mut CPU, target: InstructionSourceTarget) {
    match target {
        InstructionSourceTarget::B => cpu.registers.b = get_swap_result_and_set_flags(cpu, cpu.registers.b),
        InstructionSourceTarget::C => cpu.registers.c = get_swap_result_and_set_flags(cpu, cpu.registers.c),
        InstructionSourceTarget::D => cpu.registers.d = get_swap_result_and_set_flags(cpu, cpu.registers.d),
        InstructionSourceTarget::E => cpu.registers.e = get_swap_result_and_set_flags(cpu, cpu.registers.e),
        InstructionSourceTarget::H => cpu.registers.h = get_swap_result_and_set_flags(cpu, cpu.registers.h),
        InstructionSourceTarget::L => cpu.registers.l = get_swap_result_and_set_flags(cpu, cpu.registers.l),
        InstructionSourceTarget::HlAsPointer => panic!("This is handled in a separate function"),
        InstructionSourceTarget::A => cpu.registers.a = get_swap_result_and_set_flags(cpu, cpu.registers.a),
        _ => panic!("Source not supported"),
    };
}

pub fn swap_hl(cpu: &mut CPU, memory: &mut Memory) {
    let address: usize = cpu.registers.get_hl() as usize;
    memory.addresses[address] = get_swap_result_and_set_flags(cpu, memory.addresses[address]);
}

fn get_swap_result_and_set_flags(cpu: &mut CPU, original_value: u8) -> u8 {
    let upper_bits = original_value & 0b11110000;
    let lower_bits = original_value & 0b00001111;
    let result: u8 = (lower_bits << 4) | (upper_bits >> 4);
    set_rotate_shift_flags(cpu, result, false);
    result
}

pub fn srl(cpu: &mut CPU, target: InstructionSourceTarget) {
    match target {
        InstructionSourceTarget::B => cpu.registers.b = get_srl_result_and_set_flags(cpu, cpu.registers.b),
        InstructionSourceTarget::C => cpu.registers.c = get_srl_result_and_set_flags(cpu, cpu.registers.c),
        InstructionSourceTarget::D => cpu.registers.d = get_srl_result_and_set_flags(cpu, cpu.registers.d),
        InstructionSourceTarget::E => cpu.registers.e = get_srl_result_and_set_flags(cpu, cpu.registers.e),
        InstructionSourceTarget::H => cpu.registers.h = get_srl_result_and_set_flags(cpu, cpu.registers.h),
        InstructionSourceTarget::L => cpu.registers.l = get_srl_result_and_set_flags(cpu, cpu.registers.l),
        InstructionSourceTarget::HlAsPointer => panic!("This is handled in a separate function"),
        InstructionSourceTarget::A => cpu.registers.a = get_srl_result_and_set_flags(cpu, cpu.registers.a),
        _ => panic!("Source not supported"),
    };
}

pub fn srl_hl(cpu: &mut CPU, memory: &mut Memory) {
    let address: usize = cpu.registers.get_hl() as usize;
    memory.addresses[address] = get_srl_result_and_set_flags(cpu, memory.addresses[address]);
}

fn get_srl_result_and_set_flags(cpu: &mut CPU, original_value: u8) -> u8 {
    let carry: bool = (original_value & 0b00000001) > 0;
    let result: u8 = original_value >> 1;
    set_rotate_shift_flags(cpu, result, carry);
    result
}

pub fn bit(cpu: &mut CPU, memory: &Memory, register: InstructionSourceTarget, position: u8) {
    let byte = match position {
        0 => 0b00000001,
        1 => 0b00000010,
        2 => 0b00000100,
        3 => 0b00001000,
        4 => 0b00010000,
        5 => 0b00100000,
        6 => 0b01000000,
        7 => 0b10000000,
        _ => panic!("Invalid bit position"),
    };
    let value = match register {
        InstructionSourceTarget::B => cpu.registers.b & byte,
        InstructionSourceTarget::C => cpu.registers.c & byte,
        InstructionSourceTarget::D => cpu.registers.d & byte,
        InstructionSourceTarget::E => cpu.registers.e & byte,
        InstructionSourceTarget::H => cpu.registers.h & byte,
        InstructionSourceTarget::L => cpu.registers.l & byte,
        InstructionSourceTarget::HlAsPointer => memory.addresses[cpu.registers.get_hl() as usize] & byte,
        InstructionSourceTarget::A => cpu.registers.a & byte,
        _ => panic!("Source not supported"),
    };
    set_bit_flags(cpu, value == 0);
}

fn set_bit_flags(cpu: &mut CPU, is_bit_zero: bool) {
    // Flags: Z, 0, 1, -
    cpu.registers.set_flag_z(is_bit_zero);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(true);
}

pub fn res(cpu: &mut CPU, memory: &mut Memory, register: InstructionSourceTarget, position: u8) {
    let byte = match position {
        0 => 0b11111110,
        1 => 0b11111101,
        2 => 0b11111011,
        3 => 0b11110111,
        4 => 0b11101111,
        5 => 0b11011111,
        6 => 0b10111111,
        7 => 0b01111111,
        _ => panic!("Invalid bit position"),
    };
    match register {
        InstructionSourceTarget::B => cpu.registers.b = cpu.registers.b & byte,
        InstructionSourceTarget::C => cpu.registers.c = cpu.registers.c & byte,
        InstructionSourceTarget::D => cpu.registers.d = cpu.registers.d & byte,
        InstructionSourceTarget::E => cpu.registers.e = cpu.registers.e & byte,
        InstructionSourceTarget::H => cpu.registers.h = cpu.registers.h & byte,
        InstructionSourceTarget::L => cpu.registers.l = cpu.registers.l & byte,
        InstructionSourceTarget::HlAsPointer => {
            let address = cpu.registers.get_hl() as usize;
            memory.addresses[address] = memory.addresses[address] & byte;
        }
        InstructionSourceTarget::A => cpu.registers.a = cpu.registers.a & byte,
        _ => panic!("Source not supported"),
    };
}

pub fn set(cpu: &mut CPU, memory: &mut Memory, register: InstructionSourceTarget, position: u8) {
    let byte = match position {
        0 => 0b00000001,
        1 => 0b00000010,
        2 => 0b00000100,
        3 => 0b00001000,
        4 => 0b00010000,
        5 => 0b00100000,
        6 => 0b01000000,
        7 => 0b10000000,
        _ => panic!("Invalid bit position"),
    };
    match register {
        InstructionSourceTarget::B => cpu.registers.b = cpu.registers.b | byte,
        InstructionSourceTarget::C => cpu.registers.c = cpu.registers.c | byte,
        InstructionSourceTarget::D => cpu.registers.d = cpu.registers.d | byte,
        InstructionSourceTarget::E => cpu.registers.e = cpu.registers.e | byte,
        InstructionSourceTarget::H => cpu.registers.h = cpu.registers.h | byte,
        InstructionSourceTarget::L => cpu.registers.l = cpu.registers.l | byte,
        InstructionSourceTarget::HlAsPointer => {
            let address = cpu.registers.get_hl() as usize;
            memory.addresses[address] = memory.addresses[address] | byte;
        }
        InstructionSourceTarget::A => cpu.registers.a = cpu.registers.a | byte,
        _ => panic!("Source not supported"),
    };
}

pub fn rlca(cpu: &mut CPU) {
    rlc(cpu, InstructionSourceTarget::A);
    cpu.registers.set_flag_z(false);
    cpu.pc += 1;
}

pub fn rrca(cpu: &mut CPU) {
    rrc(cpu, InstructionSourceTarget::A);
    cpu.registers.set_flag_z(false);
    cpu.pc += 1;
}

pub fn rla(cpu: &mut CPU) {
    rl(cpu, InstructionSourceTarget::A);
    cpu.registers.set_flag_z(false);
    cpu.pc += 1;
}

pub fn rra(cpu: &mut CPU) {
    rr(cpu, InstructionSourceTarget::A);
    cpu.registers.set_flag_z(false);
    cpu.pc += 1;
}
