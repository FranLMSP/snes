use crate::ppu::registers::PPURegisters;
use crate::cpu::dma;

pub const INTERNAL_REGISTERS_ADDRESS: u16 = 0x4200;

// PPU Interrupts
pub const RDNMI: u16        = 0x4210;  // V-Blank NMI Flag

pub struct InternalRegisters {
    registers: [u8; 256],
}

impl InternalRegisters {
    pub fn new() -> Self {
        Self {
            registers: [0; 256],
        }
    }

    fn _read(&self, address: u16) -> u8 {
        self.registers[(address - 0x4200) as usize]
    }

    fn _write(&mut self, address: u16, value: u8) {
        self.registers[(address - 0x4200) as usize] = value
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

    pub fn write(&mut self, address: u16, value: u8, dma: &mut dma::DMA) {
        self._write(address, value);
        #[allow(clippy::single_match)] match address {
            dma::MDMAEN => dma.prepare_dma_transfer(value),
            _ => {},
        }
    }

    fn read_vblank_nmi(&self, ppu_registers: &PPURegisters) -> u8 {
        let byte = self._read(RDNMI);
        (byte & 0x7F) | ((ppu_registers.vblank_nmi as u8) << 7)
    }

    fn read_vblank_nmi_mut(&self, ppu_registers: &mut PPURegisters) -> u8 {
        let result = self.read_vblank_nmi(ppu_registers);
        if result == 0x80 {
            println!("nmi set");
        }
        // When register is read, bit 7 is cleared
        ppu_registers.vblank_nmi = false;
        result
    }
}

impl Default for InternalRegisters {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod ppu_general_test {
    use super::*;
    use crate::ppu::interface::PPU;

    #[test]
    fn test_read_vblank_nmi() {
        let registers = InternalRegisters::new();
        let mut ppu = PPU::new();
        ppu.registers.h_count = 20;
        ppu.dot_cycle();
        assert_eq!(registers.read_vblank_nmi(&ppu.registers), 0x00);
        ppu.registers.h_count = 339;
        ppu.registers.v_count = 224;
        ppu.dot_cycle();
        assert_eq!(registers.read_vblank_nmi_mut(&mut ppu.registers), 0x80);
        // vblank bit is reset after read
        ppu.dot_cycle();
        assert_eq!(registers.read_vblank_nmi(&ppu.registers), 0x00);
    }
}