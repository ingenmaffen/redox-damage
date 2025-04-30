use super::enums::InstructionSourceTarget;
use super::utils;
use crate::emu::cpu::CPU;
use crate::emu::memory::Memory;

pub fn ld_r8_r8(cpu: &mut CPU, memory: &mut Memory, target: InstructionSourceTarget, source: InstructionSourceTarget) {
    let value: u8 = match source {
        InstructionSourceTarget::B => cpu.registers.b,
        InstructionSourceTarget::C => cpu.registers.c,
        InstructionSourceTarget::D => cpu.registers.d,
        InstructionSourceTarget::E => cpu.registers.e,
        InstructionSourceTarget::H => cpu.registers.h,
        InstructionSourceTarget::L => cpu.registers.l,
        InstructionSourceTarget::HlAsPointer => memory.addresses[cpu.registers.get_hl() as usize],
        InstructionSourceTarget::A => cpu.registers.a,
        _ => panic!("Source register not supported"),
    };

    match target {
        InstructionSourceTarget::B => cpu.registers.b = value,
        InstructionSourceTarget::C => cpu.registers.c = value,
        InstructionSourceTarget::D => cpu.registers.d = value,
        InstructionSourceTarget::E => cpu.registers.e = value,
        InstructionSourceTarget::H => cpu.registers.h = value,
        InstructionSourceTarget::L => cpu.registers.l = value,
        InstructionSourceTarget::HlAsPointer => memory.addresses[cpu.registers.get_hl() as usize] = value,
        InstructionSourceTarget::A => cpu.registers.a = value,
        _ => panic!("Target register not supported"),
    }
    cpu.pc += 1;
}

pub fn ld_n16(cpu: &mut CPU, memory: &mut Memory, target: InstructionSourceTarget) {
    match target {
        InstructionSourceTarget::BC => cpu.registers.set_bc(utils::get_next_bytes_little_endian(cpu, memory)),
        InstructionSourceTarget::DE => cpu.registers.set_de(utils::get_next_bytes_little_endian(cpu, memory)),
        InstructionSourceTarget::HL => cpu.registers.set_hl(utils::get_next_bytes_little_endian(cpu, memory)),
        InstructionSourceTarget::SP => cpu.sp = utils::get_next_bytes_little_endian(cpu, memory),
        _ => (),
    }
    cpu.pc += 3;
}

pub fn ld_n8(cpu: &mut CPU, memory: &mut Memory, target: InstructionSourceTarget) {
    let value = memory.addresses[cpu.pc as usize + 1];
    match target {
        InstructionSourceTarget::B => cpu.registers.b = value,
        InstructionSourceTarget::C => cpu.registers.c = value,
        InstructionSourceTarget::D => cpu.registers.d = value,
        InstructionSourceTarget::E => cpu.registers.e = value,
        InstructionSourceTarget::H => cpu.registers.h = value,
        InstructionSourceTarget::L => cpu.registers.l = value,
        InstructionSourceTarget::HlAsPointer => memory.addresses[cpu.registers.get_hl() as usize] = value,
        InstructionSourceTarget::A => cpu.registers.a = value,
        _ => panic!("Target register not supported"),
    }
    cpu.pc += 2;
}

pub fn ld_a_to_pointer(cpu: &mut CPU, memory: &mut Memory, target_pointer: InstructionSourceTarget) {
    let address = match target_pointer {
        InstructionSourceTarget::BcAsPointer => cpu.registers.get_bc(),
        InstructionSourceTarget::DeAsPointer => cpu.registers.get_de(),
        InstructionSourceTarget::HlPlus => {
            let address = cpu.registers.get_hl();
            cpu.registers.set_hl(address + 1);
            address
        }
        InstructionSourceTarget::HlMinus => {
            let address = cpu.registers.get_hl();
            cpu.registers.set_hl(address + 1);
            address
        }
        _ => panic!("Target pointer not supported"),
    };
    memory.addresses[address as usize] = cpu.registers.a;
    cpu.pc += 1;
}
