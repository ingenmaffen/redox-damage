use std::fs;

#[derive(Debug)]
pub struct ROM {
    pub data: Option<Vec<u8>>,
}

impl ROM {
    pub fn load_rom_from_file(&mut self, filepath: &str) {
        self.data = match fs::read(filepath) {
            Ok(data) => Some(data),
            _ => {
                println!("Could not {}", filepath);
                None
            }
        };
    }
}
