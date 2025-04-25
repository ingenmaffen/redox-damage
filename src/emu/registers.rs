#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn set_af(&mut self, value: u16) {
        let a = value & 0xFF00;
        let f = value & 0x00FF;
        self.a = (a >> 8) as u8;
        self.f = f as u8;
    }

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | self.f as u16
    }
}
