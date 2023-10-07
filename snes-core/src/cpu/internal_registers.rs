use crate::ppu::registers::PPURegisters;

pub const INTERNAL_REGISTERS_ADDRESS: u16 = 0x4200;

// PPU Interrupts
pub const RDNMI: u16        = 0x4210;  // V-Blank NMI Flag

pub struct InternalRegisters {
    registers: [u8; 32],
}

impl InternalRegisters {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
        }
    }

    fn get_index(address: u16) -> usize {
        (address - INTERNAL_REGISTERS_ADDRESS) as usize
    }

    fn _read(&self, address: u16) -> u8 {
        let index = InternalRegisters::get_index(address);
        if index >= self.registers.len() {
            return 0xFF;
        }
        self.registers[index]
    }

    fn _write(&mut self, address: u16, value: u8) {
        let index = InternalRegisters::get_index(address);
        if index < self.registers.len() {
            self.registers[index] = value
        }
    }

    pub fn read_external(&self, address: u16, ppu_registers: &PPURegisters) -> u8 {
        match address {
            RDNMI => self.read_vblank_nmi(ppu_registers),
            _ => self._read(address),
        }
    }

    pub fn read(&self, address: u16, ppu_registers: &mut PPURegisters) -> u8 {
        match address {
            RDNMI => self.read_vblank_nmi_mut(ppu_registers),
            _ => self._read(address),
        }
    }

    pub fn read_dma(&self, address: u16) -> u8 {
        self._read(address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self._write(address, value);
    }

    fn read_vblank_nmi(&self, ppu_registers: &PPURegisters) -> u8 {
        let byte = self._read(RDNMI);
        let result = (byte & 0x7F) | ((ppu_registers.vblank_nmi as u8) << 7);
        result
    }

    fn read_vblank_nmi_mut(&self, ppu_registers: &mut PPURegisters) -> u8 {
        let byte = self._read(RDNMI);
        // When register is read, bit 7 is cleared
        let result = (byte & 0x7F) | ((ppu_registers.vblank_nmi as u8) << 7);
        ppu_registers.vblank_nmi = false;
        result
    }
}


#[cfg(test)]
mod ppu_general_test {
    use super::*;
    use crate::ppu::ppu::PPU;

    #[test]
    fn test_read_vblank_nmi() {
        let registers = InternalRegisters::new();
        let mut ppu = PPU::new();
        ppu.registers.h_count = 20;
        ppu.dot_cycle();
        assert_eq!(registers.read_vblank_nmi(&mut ppu.registers), 0x00);
        ppu.registers.h_count = 339;
        ppu.registers.v_count = 224;
        ppu.dot_cycle();
        assert_eq!(registers.read_vblank_nmi_mut(&mut ppu.registers), 0x80);
        // vblank bit is reset after read
        ppu.dot_cycle();
        assert_eq!(registers.read_vblank_nmi(&mut ppu.registers), 0x00);
    }
}