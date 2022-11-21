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
                self.registers.set_low_a(result as u8);
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
        self.increment_cycles_arithmetic(addressing_mode);
    }

    fn sbc(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
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
                    true => alu::sbc8bcd(target as u8, value, carry_flag),
                    false => alu::sbc8bin(target as u8, value, carry_flag),
                };
                self.registers.set_low_a(result as u8);
                self.registers.set_carry_flag(is_carry);
                self.registers.set_negative_flag(is_negative);
                self.registers.set_zero_flag(is_zero);
            },
            false => {
                let value = self.get_16bit_from_address(bus, addressing_mode);
                let (result, is_carry, is_negative, is_zero) = match is_decimal_mode {
                    true => alu::sbc16bcd(target, value, carry_flag),
                    false => alu::sbc16bin(target, value, carry_flag),
                };
                self.registers.a = result;
                self.registers.set_carry_flag(is_carry);
                self.registers.set_negative_flag(is_negative);
                self.registers.set_zero_flag(is_zero);
            }
        };
        self.increment_cycles_arithmetic(addressing_mode);
    }

    fn and(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        // if the M flag is set, perform 8 bit addition.
        // Otherwise, 16 bit addition
        let target = self.registers.a;
        if self.registers.is_16bit_mode() {
            let value = self.get_16bit_from_address(bus, addressing_mode);
            let (result, is_negative, is_zero) = alu::and16bit(target, value);
            self.registers.a = result;
            self.registers.set_negative_flag(is_negative);
            self.registers.set_zero_flag(is_zero);
        } else {
            let value = self.get_8bit_from_address(bus, addressing_mode);
            let (result, is_negative, is_zero) = alu::and8bit(target as u8, value);
            self.registers.set_low_a(result);
            self.registers.set_negative_flag(is_negative);
            self.registers.set_zero_flag(is_zero);

        }
        self.increment_cycles_bitwise(addressing_mode);
    }

    fn asl(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        // if the M flag is set, perform 8 bit addition.
        // Otherwise, 16 bit addition
        let target = match addressing_mode {
            AddressingMode::Accumulator => self.registers.a,
            _ => match self.registers.is_16bit_mode() {
                true => self.get_16bit_from_address(bus, addressing_mode),
                false => self.get_8bit_from_address(bus, addressing_mode) as u16,
            }
        };
        if self.registers.is_16bit_mode() {
            let (result, is_negative, is_zero, is_carry) = alu::asl16bit(target);
            self.registers.a = result;
            self.registers.set_negative_flag(is_negative);
            self.registers.set_zero_flag(is_zero);
            self.registers.set_carry_flag(is_carry);
        } else {
            let (result, is_negative, is_zero, is_carry) = alu::asl8bit(target as u8);
            self.registers.set_low_a(result);
            self.registers.set_negative_flag(is_negative);
            self.registers.set_zero_flag(is_zero);
            self.registers.set_carry_flag(is_carry);
        }
        self.increment_cycles_shift(addressing_mode);
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
            // SBC
            0xE9 => self.sbc(bus, A::Immediate),
            0xED => self.sbc(bus, A::Absolute),
            0xEF => self.sbc(bus, A::AbsoluteLong),
            0xE5 => self.sbc(bus, A::DirectPage),
            0xF2 => self.sbc(bus, A::DirectPageIndirect),
            0xE7 => self.sbc(bus, A::DirectPageIndirectLong),
            0xFD => self.sbc(bus, A::AbsoluteIndexed(I::X)),
            0xFF => self.sbc(bus, A::AbsoluteLongIndexed(I::X)),
            0xF9 => self.sbc(bus, A::AbsoluteIndexed(I::Y)),
            0xF5 => self.sbc(bus, A::DirectPageIndexed(I::X)),
            0xE1 => self.sbc(bus, A::DirectPageIndexedIndirect(I::X)),
            0xF1 => self.sbc(bus, A::DirectPageIndirectIndexed(I::Y)),
            0xF7 => self.sbc(bus, A::DirectPageIndirectLongIndexed(I::Y)),
            0xE3 => self.sbc(bus, A::StackRelative),
            0xF3 => self.sbc(bus, A::StackRelativeIndirectIndexed(I::Y)),
            // AND
            0x29 => self.and(bus, A::Immediate),
            0x2D => self.and(bus, A::Absolute),
            0x2F => self.and(bus, A::AbsoluteLong),
            0x25 => self.and(bus, A::DirectPage),
            0x32 => self.and(bus, A::DirectPageIndirect),
            0x27 => self.and(bus, A::DirectPageIndirectLong),
            0x3D => self.and(bus, A::AbsoluteIndexed(I::X)),
            0x3F => self.and(bus, A::AbsoluteLongIndexed(I::X)),
            0x39 => self.and(bus, A::AbsoluteIndexed(I::Y)),
            0x35 => self.and(bus, A::DirectPageIndexed(I::X)),
            0x21 => self.and(bus, A::DirectPageIndexedIndirect(I::X)),
            0x31 => self.and(bus, A::DirectPageIndirectIndexed(I::Y)),
            0x37 => self.and(bus, A::DirectPageIndirectLongIndexed(I::Y)),
            0x23 => self.and(bus, A::StackRelative),
            0x33 => self.and(bus, A::StackRelativeIndirectIndexed(I::Y)),
            // ASL
            0x0A => self.asl(bus, A::Accumulator),
            0x0E => self.asl(bus, A::Absolute),
            0x06 => self.asl(bus, A::DirectPage),
            0x1E => self.asl(bus, A::AbsoluteIndexed(I::X)),
            0x16 => self.asl(bus, A::DirectPageIndexed(I::X)),
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
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_carry_flag());
    }

    #[test]
    fn test_sbc() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.a   = 0x0001;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_memory_select_flag(true);
        bus.write(0x000001, 1);
        cpu.sbc(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_carry_flag());
        assert!(cpu.registers.get_zero_flag());
    }

    #[test]
    fn test_and() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.a   = 0x0101;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_memory_select_flag(true);
        bus.write(0x000001, 0x01);
        bus.write(0x000002, 0x01);
        cpu.and(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.a, 0x0101);
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_carry_flag());
        assert!(!cpu.registers.get_zero_flag());
    }

    #[test]
    fn test_asl() {
        let mut cpu = CPU::new();
        let bus = Bus::new();
        cpu.registers.a   = 0b01010000_00000000;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_memory_select_flag(false);
        cpu.asl(&bus, AddressingMode::Accumulator);
        assert_eq!(cpu.registers.a, 0b10100000_00000000);
        assert_eq!(cpu.registers.pc, 0x01);
        assert_eq!(cpu.cycles, 4);
        assert!(!cpu.registers.get_carry_flag());
        assert!(!cpu.registers.get_zero_flag());
        assert!(cpu.registers.get_negative_flag());
    }
}
