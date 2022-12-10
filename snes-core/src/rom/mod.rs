pub mod lo_rom;

use std::fs::File;
use std::io::Read;

pub fn load_rom(filename: &String, target: &mut Vec<u8>) -> std::io::Result<bool> {
    let mut file = File::open(filename)?;
    file.read_to_end(target)?;
    // TODO: header checksum here
    Ok(true)
}

pub trait ROM {
    fn load(&mut self, filename: &String) -> std::io::Result<bool>;
    fn read(&self, address: u32) -> u8;
    fn write(&mut self, address: u32, value: u8);
}