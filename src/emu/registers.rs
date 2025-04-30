#[derive(Debug, Default)]
pub struct Registers {
    pub a: u8, // Accumulator
    f: u8,     // Flags are not addressable by themselves, only as AF
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn set_af(&mut self, value: u16) {
        self.a = get_upper_byte(value);
        self.f = get_lower_byte(value);
    }

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | self.f as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = get_upper_byte(value);
        self.c = get_lower_byte(value);
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = get_upper_byte(value);
        self.e = get_lower_byte(value);
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = get_upper_byte(value);
        self.l = get_lower_byte(value);
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    pub fn set_flag_z(&mut self, value: bool) {
        self.f = set_flag_with_expression(&self.f, value, 0b10000000);
    }

    pub fn get_flag_z(&self) -> bool {
        let z = self.f & 0b10000000;
        z != 0
    }

    pub fn set_flag_n(&mut self, value: bool) {
        self.f = set_flag_with_expression(&self.f, value, 0b01000000);
    }

    pub fn get_flag_n(&self) -> bool {
        let n = self.f & 0b01000000;
        n != 0
    }

    pub fn set_flag_h(&mut self, value: bool) {
        self.f = set_flag_with_expression(&self.f, value, 0b00100000);
    }

    pub fn get_flag_h(&self) -> bool {
        let h = self.f & 0b00100000;
        h != 0
    }

    pub fn set_flag_c(&mut self, value: bool) {
        self.f = set_flag_with_expression(&self.f, value, 0b00010000);
    }

    pub fn get_flag_c(&self) -> bool {
        let c = self.f & 0b00010000;
        c != 0
    }
}

fn get_upper_byte(value: u16) -> u8 {
    ((value & 0xFF00) >> 8) as u8
}

fn get_lower_byte(value: u16) -> u8 {
    (value & 0x00FF) as u8
}

fn set_flag_with_expression(flags: &u8, value: bool, flag_id_exp: u8) -> u8 {
    if value {
        flags | flag_id_exp
    } else {
        flags & invert_bytes(flag_id_exp)
    }
}

fn invert_bytes(value: u8) -> u8 {
    value ^ 0b11111111
}
