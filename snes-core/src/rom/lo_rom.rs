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

    pub fn adjust_address(address: u32) -> u32 {
        let page = (address >> 16) & 0x7F;
        let address = address & 0x7FFF;
        (page << 16) | address
    }
}

impl ROM for LoROM {
    fn load(&mut self, filename: &String) -> std::io::Result<bool> {
        load_rom(filename, &mut self.data)
    }

    fn read(&self, address: u32) -> u8 {
        let address = LoROM::adjust_address(address);
        match self.data.get(address as usize) {
            Some(byte) => *byte,
            None => 0xFF,
        }
    }

    fn write(&mut self, _address: u32, _value: u8) {}
}