use super::cpu::CPU;
use crate::utils::addressing::{AddressingMode, IndexRegister};


impl CPU {
    pub fn increment_cycles_adc_sbc(&mut self, addressing_mode: AddressingMode) {
        type A = AddressingMode;
        type I = IndexRegister;
        let (mut bytes, mut cycles) = match addressing_mode {
            A::Immediate => (2, 2),
            A::Absolute => (3, 4),
            A::AbsoluteLong => (4, 5),
            A::DirectPage => (2, 3),
            A::DirectPageIndirect => (2, 5),
            A::DirectPageIndirectLong => (2, 6),
            A::AbsoluteIndexed(_) => (3, 4),
            A::AbsoluteLongIndexed(_) => (4, 5),
            A::DirectPageIndexed(_) => (2, 4),
            A::DirectPageIndexedIndirect(_) => (2, 6),
            A::DirectPageIndirectIndexed(_) => (2, 5),
            A::DirectPageIndirectLongIndexed(_) => (2, 6),
            A::StackRelative => (2, 4),
            A::StackRelativeIndirectIndexed(_) => (2, 7),
        };
        // condition 1: Add 1 byte and 1 cycle if m = 0 (16-bit memory/accumulator)
        if !self.registers.get_memory_select_flag() {
            bytes += 1;
            cycles += 1;
        }

        // condition 2: Add 1 cycle if low byte of Direct Page register is other than zero (DL< >0)
        match addressing_mode {
            A::DirectPage | A::DirectPageIndirect | A::DirectPageIndirectLong |
            A::DirectPageIndexed(_) | A::DirectPageIndexedIndirect(_) |
            A::DirectPageIndirectIndexed(_) | A::DirectPageIndirectLongIndexed(_) => {
                if self.registers.direct_page_low() != 0 {
                    cycles += 1;
                }
            },
            _ => {},
        };

        // condition 3: Add 1 cycle if adding index crosses a page boundary
        match addressing_mode {
            A::AbsoluteIndexed(index) | A::DirectPageIndirectIndexed(index) => {
                let page = (self.registers.get_pc_address() - (bytes as u32)) & 0xFF;
                let index = match index {
                    I::X => self.registers.x,
                    I::Y => self.registers.y,
                };
                if (page + index as u32) > 0xFF {
                    cycles += 1
                }
            },
            _ => {},
        };

        // condition 4: Add 1 cycle if 65C02 and d = 1 (decimal mode, 65C02)
        if self.registers.get_decimal_mode_flag() {
            cycles += 1;
        }

        self.registers.increment_pc(bytes);
        self.cycles += cycles;
    }
}

#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test_increment_cycles_adc_sbc() {
        let mut cpu = CPU::new();

        // 16-bit Memory/accumulator flag condition
        cpu.registers.pc = 0;
        cpu.cycles = 0;
        cpu.registers.set_decimal_mode_flag(false);
        cpu.registers.set_16bit_mode(false);
        cpu.increment_cycles_adc_sbc(AddressingMode::Immediate);
        assert_eq!(cpu.registers.pc, 2);
        assert_eq!(cpu.cycles, 2);

        cpu.registers.pc = 0;
        cpu.cycles = 0;
        cpu.registers.set_16bit_mode(true);
        cpu.increment_cycles_adc_sbc(AddressingMode::Immediate);
        assert_eq!(cpu.registers.pc, 3);
        assert_eq!(cpu.cycles, 3);

        // Decimal flag condition
        cpu.registers.pc = 0;
        cpu.cycles = 0;
        cpu.registers.set_16bit_mode(true);
        cpu.registers.set_decimal_mode_flag(true);
        cpu.increment_cycles_adc_sbc(AddressingMode::Immediate);
        assert_eq!(cpu.registers.pc, 3);
        assert_eq!(cpu.cycles, 4);

        // Low byte of direct page register other than zero condition
        cpu.registers.pc = 0;
        cpu.cycles = 0;
        cpu.registers.set_16bit_mode(false);
        cpu.registers.set_decimal_mode_flag(false);
        cpu.registers.d = 0x0000;
        cpu.increment_cycles_adc_sbc(AddressingMode::DirectPage);
        assert_eq!(cpu.registers.pc, 2);
        assert_eq!(cpu.cycles, 3);

        cpu.registers.pc = 0;
        cpu.cycles = 0;
        cpu.registers.set_16bit_mode(false);
        cpu.registers.set_decimal_mode_flag(false);
        cpu.registers.d = 0x0001;
        cpu.increment_cycles_adc_sbc(AddressingMode::DirectPage);
        assert_eq!(cpu.registers.pc, 2);
        assert_eq!(cpu.cycles, 4);

        // Adding index crosses a page boundary condition
        cpu.registers.pc = 0xFE;
        cpu.registers.x = 0x0001;
        cpu.cycles = 0;
        cpu.registers.set_16bit_mode(false);
        cpu.registers.set_decimal_mode_flag(false);
        cpu.increment_cycles_adc_sbc(AddressingMode::AbsoluteIndexed(IndexRegister::X));
        assert_eq!(cpu.registers.pc, 0xFE + 3);
        assert_eq!(cpu.cycles, 4);

        cpu.registers.pc = 0xFE;
        cpu.registers.x = 0x0010;
        cpu.cycles = 0;
        cpu.registers.set_16bit_mode(false);
        cpu.registers.set_decimal_mode_flag(false);
        cpu.increment_cycles_adc_sbc(AddressingMode::AbsoluteIndexed(IndexRegister::X));
        assert_eq!(cpu.registers.pc, 0xFE + 3);
        assert_eq!(cpu.cycles, 5);
    }
}