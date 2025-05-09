use super::enums::InstructionSourceTarget;
use crate::emu::cpu::CPU;
use crate::emu::instructions::utils;
use crate::emu::memory::Memory;

pub fn dec(cpu: &mut CPU, target: InstructionSourceTarget) {
    match target {
        InstructionSourceTarget::BC => cpu.registers.set_bc(get_new_value_after_dec_u16(cpu.registers.get_bc())),
        InstructionSourceTarget::DE => cpu.registers.set_de(get_new_value_after_dec_u16(cpu.registers.get_de())),
        InstructionSourceTarget::HL => cpu.registers.set_hl(get_new_value_after_dec_u16(cpu.registers.get_hl())),
        InstructionSourceTarget::SP => cpu.sp = get_new_value_after_dec_u16(cpu.sp),
        InstructionSourceTarget::HlAsPointer => panic!("This is handled in a separate function"),
        InstructionSourceTarget::BcAsPointer => panic!("Target not supported"),
        InstructionSourceTarget::DeAsPointer => panic!("Target not supported"),
        InstructionSourceTarget::HlMinus => panic!("Target not supported"),
        InstructionSourceTarget::HlPlus => panic!("Target not supported"),
        _ => dec_r8(cpu, target),
    }
    cpu.pc += 1;
}

pub fn dec_r8_at_hl(cpu: &mut CPU, memory: &mut Memory) {
    let address: usize = cpu.registers.get_hl() as usize;
    let value = get_new_value_after_dec(utils::read_byte_from_memory(memory, address));
    utils::write_byte_to_memory(memory, address, value);
    set_dec_flags(value, cpu);
}

fn dec_r8(cpu: &mut CPU, target: InstructionSourceTarget) {
    match target {
        InstructionSourceTarget::B => {
            cpu.registers.b = get_new_value_after_dec(cpu.registers.b);
            set_dec_flags(cpu.registers.b, cpu);
        }
        InstructionSourceTarget::C => {
            cpu.registers.c = get_new_value_after_dec(cpu.registers.c);
            set_dec_flags(cpu.registers.c, cpu);
        }
        InstructionSourceTarget::D => {
            cpu.registers.d = get_new_value_after_dec(cpu.registers.d);
            set_dec_flags(cpu.registers.d, cpu);
        }
        InstructionSourceTarget::E => {
            cpu.registers.e = get_new_value_after_dec(cpu.registers.e);
            set_dec_flags(cpu.registers.e, cpu);
        }
        InstructionSourceTarget::H => {
            cpu.registers.h = get_new_value_after_dec(cpu.registers.h);
            set_dec_flags(cpu.registers.h, cpu);
        }
        InstructionSourceTarget::L => {
            cpu.registers.l = get_new_value_after_dec(cpu.registers.l);
            set_dec_flags(cpu.registers.l, cpu);
        }
        InstructionSourceTarget::A => {
            cpu.registers.a = get_new_value_after_dec(cpu.registers.a);
            set_dec_flags(cpu.registers.a, cpu);
        }
        _ => panic!("Unhandled case"),
    }
}

fn get_new_value_after_dec(original_value: u8) -> u8 {
    if original_value == 0 {
        return u8::MAX;
    }
    return original_value - 1;
}

fn get_new_value_after_dec_u16(original_value: u16) -> u16 {
    if original_value == 0 {
        return u16::MAX;
    }
    return original_value - 1;
}

fn set_dec_flags(new_value: u8, cpu: &mut CPU) {
    // Flags: Z, 1, H, -
    cpu.registers.set_flag_z(new_value == 0);
    cpu.registers.set_flag_n(true);
    cpu.registers.set_flag_h((new_value & 0x0F) == 0);
}
