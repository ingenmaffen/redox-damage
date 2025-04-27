#[derive(Debug)]
pub struct Memory {
    pub addresses: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            addresses: initialize_memory(),
        }
    }
}

fn initialize_memory() -> Vec<u8> {
    let len: usize = u16::MAX as usize;
    vec![0; len]
}

impl Memory {
    pub fn load_rom_data_into_bank_00(&mut self, romdata: &Vec<u8>) {
        for i in 0x0000..=0x3FFF {
            self.addresses[i] = romdata[i];
        }
    }

    pub fn load_rom_data_into_bank_01(&mut self, romdata: &Vec<u8>) {
        let cartridge_type: u8 = romdata[0x0147];

        // 0x00 means ROM only
        // and currently it only supports ROM only games
        if cartridge_type != 0x00 {
            panic!("ROM not supported")
        }
        for i in 0x4000..=0x7FFF {
            self.addresses[i] = romdata[i];
        }
    }
}
