use super::registers::Registers;

#[derive(Debug, Default)]
pub struct CPU {
    pub registers: Registers,
    pub sp: u16, // Stack Pointer
    pub pc: u16, // Program Counter
}
