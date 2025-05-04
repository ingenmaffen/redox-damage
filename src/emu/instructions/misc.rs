use crate::emu::cpu::CPU;
use crate::emu::memory::Memory;

pub fn nop(cpu: &mut CPU) {
    cpu.pc += 1;
}

pub fn stop(cpu: &mut CPU, memory: &Memory) {
    // TODO
    cpu.pc += 2;
}

pub fn halt(cpu: &mut CPU) {
    // TODO
    cpu.pc += 1;
}

pub fn di(cpu: &mut CPU) {
    // TODO
    cpu.pc += 1;
}

pub fn ei(cpu: &mut CPU) {
    // TODO
    cpu.pc += 1;
}

pub fn daa(cpu: &mut CPU) {
    let mut adjusment: u8 = 0;
    if cpu.registers.get_flag_n() {
        if cpu.registers.get_flag_h() {
            adjusment += 0x06;
        }
        if cpu.registers.get_flag_c() {
            adjusment += 0x60;
        }
        cpu.registers.a -= adjusment;
    } else {
        if cpu.registers.get_flag_h() || cpu.registers.a > 0x09 {
            adjusment += 0x06;
        }
        if cpu.registers.get_flag_c() || cpu.registers.a > 0x99 {
            adjusment += 0x60;
            cpu.registers.set_flag_c(true);
        }
        cpu.registers.a -= adjusment;
    }
    cpu.registers.set_flag_z(cpu.registers.a == 0);
    cpu.registers.set_flag_h(false);
    cpu.pc += 1;
}

pub fn cpl(cpu: &mut CPU) {
    cpu.registers.a = cpu.registers.a ^ 0xFF;
    cpu.pc += 1;
}

pub fn scf(cpu: &mut CPU) {
    cpu.registers.set_flag_c(true);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_n(false);
    cpu.pc += 1;
}

pub fn ccf(cpu: &mut CPU) {
    cpu.registers.set_flag_c(!cpu.registers.get_flag_c());
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_n(false);
    cpu.pc += 1;
}
