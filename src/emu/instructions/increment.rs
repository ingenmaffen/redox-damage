use crate::emu::cpu::CPU;
use crate::emu::memory::Memory;

use super::enums::InstructionSourceTarget;

pub fn inc(cpu: &mut CPU, target: InstructionSourceTarget) {
    match target {
        InstructionSourceTarget::BC => cpu.registers.set_bc(cpu.registers.get_bc() + 1),
        InstructionSourceTarget::DE => cpu.registers.set_de(cpu.registers.get_de() + 1),
        InstructionSourceTarget::HL => cpu.registers.set_hl(cpu.registers.get_hl() + 1),
        InstructionSourceTarget::SP => cpu.sp += 1,
        InstructionSourceTarget::HlAsPointer => panic!("This is handled in a separate function"),
        _ => inc_r8(cpu, target),
    }
    cpu.pc += 1;
}

pub fn inc_r8_at_hl(cpu: &mut CPU, memory: &mut Memory) {
    let address: usize = cpu.registers.get_hl() as usize;
    memory.addresses[address] += 1;
    let new_value = memory.addresses[address];
    set_inc_flags(new_value, cpu);
}

fn inc_r8(cpu: &mut CPU, target: InstructionSourceTarget) {
    match target {
        InstructionSourceTarget::B => {
            cpu.registers.b += 1;
            let new_value = cpu.registers.b;
            set_inc_flags(new_value, cpu);
        }
        InstructionSourceTarget::C => {
            cpu.registers.c += 1;
            let new_value = cpu.registers.c;
            set_inc_flags(new_value, cpu);
        }
        InstructionSourceTarget::D => {
            cpu.registers.d += 1;
            let new_value = cpu.registers.d;
            set_inc_flags(new_value, cpu);
        }
        InstructionSourceTarget::E => {
            cpu.registers.e += 1;
            let new_value = cpu.registers.e;
            set_inc_flags(new_value, cpu);
        }
        InstructionSourceTarget::H => {
            cpu.registers.h += 1;
            let new_value = cpu.registers.h;
            set_inc_flags(new_value, cpu);
        }
        InstructionSourceTarget::L => {
            cpu.registers.l += 1;
            let new_value = cpu.registers.l;
            set_inc_flags(new_value, cpu);
        }
        InstructionSourceTarget::A => {
            cpu.registers.a += 1;
            let new_value = cpu.registers.a;
            set_inc_flags(new_value, cpu);
        }
        _ => (),
    }
}

fn set_inc_flags(new_value: u8, cpu: &mut CPU) {
    // Flags: Z, 0, H, -
    cpu.registers.set_flag_z(new_value == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h((new_value & 0x0F) == 0);
}
