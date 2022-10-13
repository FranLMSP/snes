use super::cpu::CPU;
use crate::bus::Bus;
use crate::utils::addressing::{AddressingMode, IndexRegister};
use crate::utils::alu;

impl CPU {
    fn get_8bit_from_address(&self, bus: &Bus, addressing_mode: AddressingMode) -> u8 {
        addressing_mode.value_8bit(
            bus,
            self.registers.get_pc_address(),
            self.registers.d,
            self.registers.sp,
            self.registers.x, self.registers.y
        )
    }

    fn get_16bit_from_address(&self, bus: &Bus, addressing_mode: AddressingMode) -> u16 {
        addressing_mode.value_16bit(
            bus,
            self.registers.get_pc_address(),
            self.registers.d,
            self.registers.sp,
            self.registers.x, self.registers.y
        )
    }

    fn adc(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        // if the M flag is set, perform 8 bit addition.
        // Otherwise, 16 bit addition
        let carry_flag = self.registers.get_carry_flag();
        let is_decimal_mode = self.registers.get_decimal_mode_flag();
        let is_8bit = self.registers.get_memory_select_flag();
        let target = self.registers.a;
        match is_8bit {
            true => {
                let value = self.get_8bit_from_address(bus, addressing_mode);
                let (result, is_carry, is_negative, is_zero) = match is_decimal_mode {
                    true => alu::adc8bcd(target as u8, value, carry_flag),
                    false => alu::adc8bin(target as u8, value, carry_flag),
                };
                self.registers.a = result as u16;
                self.registers.set_carry_flag(is_carry);
                self.registers.set_negative_flag(is_negative);
                self.registers.set_zero_flag(is_zero);
            },
            false => {
                let value = self.get_16bit_from_address(bus, addressing_mode);
                let (result, is_carry, is_negative, is_zero) = match is_decimal_mode {
                    true => alu::adc16bcd(target, value, carry_flag),
                    false => alu::adc16bin(target, value, carry_flag),
                };
                self.registers.a = result;
                self.registers.set_carry_flag(is_carry);
                self.registers.set_negative_flag(is_negative);
                self.registers.set_zero_flag(is_zero);
            }
        };
    }

    pub fn execute_opcode(&mut self, opcode: u8, bus: &Bus) {
        type A = AddressingMode;
        type I = IndexRegister;
        match opcode {
            // ADC
            0x69 => self.adc(bus, A::Immediate),
            0x6D => self.adc(bus, A::Absolute),
            0x6F => self.adc(bus, A::AbsoluteLong),
            0x65 => self.adc(bus, A::DirectPage),
            0x72 => self.adc(bus, A::DirectPageIndirect),
            0x67 => self.adc(bus, A::DirectPageIndirectLong),
            0x7D => self.adc(bus, A::AbsoluteIndexed(I::X)),
            0x7F => self.adc(bus, A::AbsoluteLongIndexed(I::X)),
            0x79 => self.adc(bus, A::AbsoluteIndexed(I::Y)),
            0x75 => self.adc(bus, A::DirectPageIndexed(I::X)),
            0x61 => self.adc(bus, A::DirectPageIndexedIndirect(I::X)),
            0x71 => self.adc(bus, A::DirectPageIndirectIndexed(I::Y)),
            0x77 => self.adc(bus, A::DirectPageIndirectLongIndexed(I::Y)),
            0x63 => self.adc(bus, A::StackRelative),
            0x73 => self.adc(bus, A::StackRelativeIndirectIndexed(I::Y)),
            _ => println!("Invalid opcode: {:02X}", opcode),
        }
    }
}

#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test_adc() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.a   = 0x0000;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_memory_select_flag(true);
        bus.write(0x000001, 0x40);
        cpu.adc(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.a, 0x40);
        assert!(!cpu.registers.get_carry_flag());
    }
}
