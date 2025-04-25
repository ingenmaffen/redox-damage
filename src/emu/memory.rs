#[derive(Debug)]
pub struct Memory {
    pub registers: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            registers: initialize_memory(),
        }
    }
}

fn initialize_memory() -> Vec<u8> {
    let len: usize = u16::MAX as usize;
    vec![0; len]
}

impl Memory {
    pub fn load_rom_data_into_bank_00(&mut self, romdata: Vec<u8>) {
        for byte in romdata {
            print!("{:2X} ", &byte)
        }
    }

    pub fn load_rom_data_into_bank_01(&mut self, romdata: Vec<u8>) {
        for byte in romdata {
            print!("{:2X} ", &byte)
        }
    }
}
