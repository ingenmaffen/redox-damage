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
        InstructionSourceTarget::HlAsPointer => utils::read_byte_from_memory(memory, cpu.registers.get_hl() as usize),
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
        InstructionSourceTarget::HlAsPointer => utils::write_byte_to_memory(memory, cpu.registers.get_hl() as usize, value),
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
    let value = utils::read_byte_from_memory(memory, cpu.pc as usize + 1);
    match target {
        InstructionSourceTarget::B => cpu.registers.b = value,
        InstructionSourceTarget::C => cpu.registers.c = value,
        InstructionSourceTarget::D => cpu.registers.d = value,
        InstructionSourceTarget::E => cpu.registers.e = value,
        InstructionSourceTarget::H => cpu.registers.h = value,
        InstructionSourceTarget::L => cpu.registers.l = value,
        InstructionSourceTarget::HlAsPointer => utils::write_byte_to_memory(memory, cpu.registers.get_hl() as usize, value),
        InstructionSourceTarget::A => cpu.registers.a = value,
        _ => panic!("Target register not supported"),
    }
    cpu.pc += 2;
}

pub fn ld_a_to_pointer(cpu: &mut CPU, memory: &mut Memory, target_pointer: InstructionSourceTarget) {
    let address = match target_pointer {
        InstructionSourceTarget::BcAsPointer => cpu.registers.get_bc(),
        InstructionSourceTarget::DeAsPointer => cpu.registers.get_de(),
        InstructionSourceTarget::CAsPointer => 0xFF00 | cpu.registers.c as u16,
        InstructionSourceTarget::HlPlus => {
            let address = cpu.registers.get_hl();
            if cpu.registers.get_hl() == u16::MAX {
                cpu.registers.set_hl(0);
            } else {
                cpu.registers.set_hl(address + 1);
            }
            address
        }
        InstructionSourceTarget::HlMinus => {
            let address = cpu.registers.get_hl();
            if cpu.registers.get_hl() == 0 {
                cpu.registers.set_hl(u16::MAX);
            } else {
                cpu.registers.set_hl(address - 1);
            }
            address
        }
        _ => panic!("Target pointer not supported"),
    };
    utils::write_byte_to_memory(memory, address as usize, cpu.registers.a);
    cpu.pc += 1;
}

pub fn ld_pointer_to_a(cpu: &mut CPU, memory: &Memory, source_pointer: InstructionSourceTarget) {
    let address = match source_pointer {
        InstructionSourceTarget::BcAsPointer => cpu.registers.get_bc(),
        InstructionSourceTarget::DeAsPointer => cpu.registers.get_de(),
        InstructionSourceTarget::CAsPointer => 0xFF00 | cpu.registers.c as u16,
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
        _ => panic!("Source pointer not supported"),
    };
    cpu.registers.a = utils::read_byte_from_memory(memory, address as usize);
    cpu.pc += 1;
}

pub fn ld_sp_to_n16(cpu: &mut CPU, memory: &mut Memory) {
    let address: usize = utils::get_next_bytes_little_endian(cpu, memory) as usize;
    utils::write_byte_to_memory(memory, address, (cpu.sp & 0xFF) as u8);
    utils::write_byte_to_memory(memory, address + 1, (cpu.sp >> 8) as u8);
    cpu.pc += 3;
}

pub fn ld_sp_and_e8_to_hl(cpu: &mut CPU, memory: &Memory) {
    let value: i8 = utils::get_e8(cpu, memory);
    if value < 0 {
        cpu.sp -= value.abs() as u16;
    } else {
        cpu.sp += value as u16;
    }
    cpu.registers.set_hl(cpu.sp);
    cpu.pc += 2;
}

pub fn pop(cpu: &mut CPU, memory: &Memory, target: InstructionSourceTarget) {
    let value1 = utils::read_byte_from_memory(memory, cpu.pc as usize) as u16;
    let value2 = utils::read_byte_from_memory(memory, cpu.pc as usize + 1) as u16;
    let value: u16 = value2 << 8 | value1;
    match target {
        InstructionSourceTarget::BC => cpu.registers.set_bc(value),
        InstructionSourceTarget::DE => cpu.registers.set_de(value),
        InstructionSourceTarget::HL => cpu.registers.set_hl(value),
        InstructionSourceTarget::AF => cpu.registers.set_af(value),
        _ => panic!("Target not supported"),
    }
    cpu.sp += 2;
    cpu.pc += 1;
}

pub fn push(cpu: &mut CPU, memory: &mut Memory, source: InstructionSourceTarget) {
    let values: (u8, u8) = match source {
        InstructionSourceTarget::BC => (cpu.registers.b, cpu.registers.c),
        InstructionSourceTarget::DE => (cpu.registers.d, cpu.registers.e),
        InstructionSourceTarget::HL => (cpu.registers.h, cpu.registers.l),
        InstructionSourceTarget::AF => (cpu.registers.a, cpu.registers.get_f()),
        _ => panic!("Target not supported"),
    };
    utils::write_byte_to_memory(memory, cpu.sp as usize, values.0);
    utils::write_byte_to_memory(memory, cpu.sp as usize - 1, values.1);
    cpu.sp -= 2;
    cpu.pc += 1;
}

pub fn ld_a16_to_a(cpu: &mut CPU, memory: &Memory) {
    let pointer = utils::get_next_bytes_little_endian(cpu, memory) as usize;
    cpu.registers.a = utils::read_byte_from_memory(memory, pointer);
    cpu.pc += 3;
}

pub fn ld_a_to_a16(cpu: &mut CPU, memory: &mut Memory) {
    let pointer = utils::get_next_bytes_little_endian(cpu, memory) as usize;
    utils::write_byte_to_memory(memory, pointer, cpu.registers.a);
    cpu.pc += 3;
}

pub fn ldh_a8_to_a(cpu: &mut CPU, memory: &Memory) {
    let pointer: usize = 0xFF00 | utils::read_byte_from_memory(memory, cpu.pc as usize + 1) as usize;
    cpu.registers.a = utils::read_byte_from_memory(memory, pointer);
    cpu.pc += 2;
}

pub fn ldh_a_to_a8(cpu: &mut CPU, memory: &mut Memory) {
    let pointer: usize = 0xFF00 | utils::read_byte_from_memory(memory, cpu.pc as usize + 1) as usize;
    utils::write_byte_to_memory(memory, pointer, cpu.registers.a);
    cpu.pc += 2;
}

pub fn ld_hl_to_sp(cpu: &mut CPU) {
    cpu.sp = cpu.registers.get_hl();
    cpu.pc += 1;
}
