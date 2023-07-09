use crate::ppu::PPU;
use crate::cpu::internal_registers::InternalRegisters;
use crate::rom::ROM;
use crate::rom::lo_rom::LoROM;

pub struct Bus {
    wram: [u8; 0x10000],
    pub ppu: PPU,
    pub rom: Box<dyn ROM>,
    pub internal_registers: InternalRegisters,
}

#[derive(PartialEq, Debug)]
pub enum MemoryMap {
    WRAM,
    PPU,
    CPU,
    Joypad,
    Cartridge,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            wram: [0; 0x10000],
            ppu: PPU::new(),
            rom: Box::new(LoROM::new()),
            internal_registers: InternalRegisters::new(),
        }
    }

    fn read_wram(&self, address: u32) -> u8 {
        return self.wram[(address & 0xFFFF) as usize];
    }

    fn write_wram(&mut self, address: u32, value: u8) {
        self.wram[(address & 0xFFFF) as usize] = value;
    }

    fn map_address(address: u32) -> MemoryMap {
        let (bank, sub_address) = {
            let bank = (address >> 16) as u8;
            let sub_address = address as u16;
            (bank, sub_address)
        };
        match bank {
            0x7E..=0x7F => MemoryMap::WRAM,
            0x80..=0xBF | 0x00..=0x3F => match sub_address {
                0x0000..=0x1FFF => MemoryMap::WRAM,
                0x2100..=0x21FF => MemoryMap::PPU,
                0x4016..=0x4017 => MemoryMap::Joypad,
                0x4200..=0x42FF => MemoryMap::CPU,
                _ => MemoryMap::Cartridge,
            },
            _ => MemoryMap::Cartridge,
        }
    }

    /// This function is meant to be used by external parts of the code,
    /// for example, to render register info without mutating them
    pub fn read_external(&self, address: u32) -> u8 {
        let section = Bus::map_address(address);
        match section {
            MemoryMap::WRAM => self.read_wram(address),
            MemoryMap::PPU => self.ppu.registers.read_external(address as u16),
            MemoryMap::CPU => self.internal_registers.read_external(
                address as u16,
                &self.ppu.registers,
            ),
            MemoryMap::Joypad => 0x00,  // TODO: Placeholder
            MemoryMap::Cartridge => self.rom.read(address),
        }
    }

    pub fn read(&mut self, address: u32) -> u8 {
        let section = Bus::map_address(address);
        match section {
            MemoryMap::WRAM => self.read_wram(address),
            MemoryMap::PPU => self.ppu.registers.read(address as u16),
            MemoryMap::CPU => self.internal_registers.read(
                address as u16,
                &mut self.ppu.registers,
            ),
            MemoryMap::Joypad => 0x00,  // TODO: Placeholder
            MemoryMap::Cartridge => self.rom.read(address),
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        let section = Bus::map_address(address);
        match section {
            MemoryMap::WRAM => self.write_wram(address, value),
            MemoryMap::PPU => self.ppu.registers.write(address as u16, value),
            MemoryMap::CPU => self.internal_registers.write(address as u16, value),
            MemoryMap::Joypad => {},  // TODO: Placeholder
            MemoryMap::Cartridge => self.rom.write(address, value),
        }
    }
}


#[cfg(test)]
mod bus_tests {
    use super::*;

    #[test]
    fn test_memory_map() {
        assert_eq!(Bus::map_address(0x7E0000), MemoryMap::WRAM);
        assert_eq!(Bus::map_address(0x7F0000), MemoryMap::WRAM);
        assert_eq!(Bus::map_address(0x7E0500), MemoryMap::WRAM);
        assert_eq!(Bus::map_address(0x000000), MemoryMap::WRAM);
        assert_eq!(Bus::map_address(0x3F0000), MemoryMap::WRAM);
        assert_eq!(Bus::map_address(0x3F1FFF), MemoryMap::WRAM);
        assert_eq!(Bus::map_address(0x3F0000), MemoryMap::WRAM);
        assert_eq!(Bus::map_address(0x800000), MemoryMap::WRAM);
        assert_eq!(Bus::map_address(0xBF0000), MemoryMap::WRAM);
        assert_eq!(Bus::map_address(0xBF1FFF), MemoryMap::WRAM);
        assert_eq!(Bus::map_address(0x3F0000), MemoryMap::WRAM);

        assert_eq!(Bus::map_address(0x002100), MemoryMap::PPU);
        assert_eq!(Bus::map_address(0x0021FF), MemoryMap::PPU);
        assert_eq!(Bus::map_address(0x3F2100), MemoryMap::PPU);
        assert_eq!(Bus::map_address(0x3F21FF), MemoryMap::PPU);
        assert_eq!(Bus::map_address(0x802100), MemoryMap::PPU);
        assert_eq!(Bus::map_address(0x8021FF), MemoryMap::PPU);
        assert_eq!(Bus::map_address(0xBF2100), MemoryMap::PPU);
        assert_eq!(Bus::map_address(0xBF21FF), MemoryMap::PPU);

        assert_eq!(Bus::map_address(0x004200), MemoryMap::CPU);
        assert_eq!(Bus::map_address(0x00420F), MemoryMap::CPU);
        assert_eq!(Bus::map_address(0x3F4200), MemoryMap::CPU);
        assert_eq!(Bus::map_address(0x3F420F), MemoryMap::CPU);
        assert_eq!(Bus::map_address(0x804200), MemoryMap::CPU);
        assert_eq!(Bus::map_address(0x80420F), MemoryMap::CPU);
        assert_eq!(Bus::map_address(0xBF4200), MemoryMap::CPU);
        assert_eq!(Bus::map_address(0xBF420F), MemoryMap::CPU);

        assert_eq!(Bus::map_address(0x004016), MemoryMap::Joypad);
        assert_eq!(Bus::map_address(0x004017), MemoryMap::Joypad);
        assert_eq!(Bus::map_address(0x3F4016), MemoryMap::Joypad);
        assert_eq!(Bus::map_address(0x3F4017), MemoryMap::Joypad);
        assert_eq!(Bus::map_address(0x804016), MemoryMap::Joypad);
        assert_eq!(Bus::map_address(0x804017), MemoryMap::Joypad);
        assert_eq!(Bus::map_address(0xBF4016), MemoryMap::Joypad);
        assert_eq!(Bus::map_address(0xBF4017), MemoryMap::Joypad);
    }

    #[test]
    fn test_wram_mirror() {
        let mut bus = Bus::new();
        bus.write(0x00_0000, 0x1F);
        assert_eq!(bus.read(0x7E_0000), 0x1F);
        assert_eq!(bus.read(0x80_0000), 0x1F);
        bus.write(0x80_0000, 0xEE);
        assert_eq!(bus.read(0x7E_0000), 0xEE);
        assert_eq!(bus.read(0x00_0000), 0xEE);
    }
}

