use crate::emu::cpu::CPU;

pub fn nop(cpu: &mut CPU) {
    cpu.pc += 1;
}
