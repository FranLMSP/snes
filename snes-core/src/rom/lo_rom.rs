use super::{ROM, load_rom};

pub struct LoROM {
    data: Vec<u8>,
}

impl LoROM {
    pub fn new() -> Self {
        Self {
            data: vec![],
        }
    }
}

impl ROM for LoROM {
    fn load(&mut self, filename: &String) -> std::io::Result<bool> {
        load_rom(filename, &mut self.data)
    }

    fn read(&self, address: u32) -> u8 {
        match self.data.get(address as usize) {
            Some(byte) => *byte,
            None => 0xFF,
        }
    }

    fn write(&mut self, _address: u32, _value: u8) {}
}