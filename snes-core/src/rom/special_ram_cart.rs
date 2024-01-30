use super::ROM;

pub struct SpecialRAMCart {
    data: Vec<u8>,
}

impl SpecialRAMCart {
    pub fn new() -> Self {
        Self {
            data: vec![0x00; 0x100000000],
        }
    }
}

impl ROM for SpecialRAMCart {
    fn load(&mut self, _filename: &str) -> std::io::Result<bool> {
        Ok(true)
    }

    fn read(&self, address: u32) -> u8 {
        match self.data.get(address as usize) {
            Some(byte) => *byte,
            None => 0x00,
        }
    }

    fn write(&mut self, address: u32, value: u8) {
        self.data[address as usize] = value;
    }
}

impl Default for SpecialRAMCart {
    fn default() -> Self {
        Self::new()
    }
}
