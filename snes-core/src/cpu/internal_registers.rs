use crate::ppu::registers::PPURegisters;
use crate::ppu::registers::{
    RDNMI
};

pub const INTERNAL_REGISTERS_ADDRESS: u16 = 0x4200;

pub struct InternalRegisters {
    registers: [u8; 32],
}

impl InternalRegisters {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
        }
    }

    fn _read(&self, address: u16) -> u8 {
        self.registers[(address - INTERNAL_REGISTERS_ADDRESS) as usize]
    }

    fn _write(&mut self, address: u16, value: u8) {
        self.registers[(address - INTERNAL_REGISTERS_ADDRESS) as usize] = value
    }

    pub fn read(&self, address: u16, ppu_registers: &PPURegisters) -> u8 {
        match address {
            RDNMI => self.read_vblank_nmi(ppu_registers),
            _ => self._read(address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self._write(address, value);
    }

    fn read_vblank_nmi(&self, ppu_registers: &PPURegisters) -> u8 {
        let byte = self._read(RDNMI);
        // TODO: when this register is read, bit 7 is cleared
        (byte & 0x7F) | ((ppu_registers.is_vblanking() as u8) << 7)
    }
}


#[cfg(test)]
mod ppu_general_test {
    use super::*;

    #[test]
    fn test_read_vblank_nmi() {
        let registers = InternalRegisters::new();
        let mut ppu_registers = PPURegisters::new();
        ppu_registers.h_count = 20;
        assert_eq!(registers.read_vblank_nmi(&ppu_registers), 0x00);
        ppu_registers.h_count = 300;
        assert_eq!(registers.read_vblank_nmi(&ppu_registers), 0x80);
        // TODO: reset vblank bit after read
        // ppu_registers.h_count = 300;
        // assert_eq!(registers.read_vblank_nmi(&ppu_registers), 0x00);
    }
}