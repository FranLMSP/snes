use crate::{cpu::{bus::Bus, registers::Registers}, utils::{alu, addressing::AddressingMode}};

use crate::cpu::cycles;
use super::{CPUInstruction, read_write_common::{read_8bit_from_address, read_16bit_from_address}};
use super::decoder_common;

static INSTR_NAME: &'static str = "ADC";

pub struct ADC {
    pub addressing_mode: AddressingMode,
}

impl ADC {
    fn determine_instruction(&self, registers: &Registers) -> Box<dyn CPUInstruction> {
        let is_decimal_mode = registers.get_decimal_mode_flag();
        match registers.is_16bit_mode() {
            true => match is_decimal_mode {
                true => Box::new(ADC16BCD{addressing_mode: self.addressing_mode}),
                false => Box::new(ADC16BIN{addressing_mode: self.addressing_mode}),
            }
            false => match is_decimal_mode {
                true => Box::new(ADC8BCD{addressing_mode: self.addressing_mode}),
                false => Box::new(ADC8BIN{addressing_mode: self.addressing_mode}),
            }
        }
    }
}

impl CPUInstruction for ADC {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let instruction = self.determine_instruction(registers);
        instruction.execute(registers, bus);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        let instruction = self.determine_instruction(registers);
        instruction.mnemonic(registers, bus, opcode)
    }
}

pub struct ADC8BIN {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for ADC8BIN {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let (result, affected_flags) = alu::adc_bin(
            registers.a as u8,
            read_8bit_from_address(registers, bus, self.addressing_mode),
            registers.get_carry_flag(),
        );
        registers.set_low_a(result);
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_arithmetic(&registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct ADC16BIN {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for ADC16BIN {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let (result, affected_flags) = alu::adc_bin(
            registers.a,
            read_16bit_from_address(registers, bus, self.addressing_mode),
            registers.get_carry_flag(),
        );
        registers.a = result;
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_arithmetic(&registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(true, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct ADC8BCD {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for ADC8BCD {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let (result, affected_flags) = alu::adc_bcd(
            registers.a as u8,
            read_8bit_from_address(registers, bus, self.addressing_mode),
            registers.get_carry_flag(),
        );
        registers.set_low_a(result);
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_arithmetic(&registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct ADC16BCD {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for ADC16BCD {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let (result, affected_flags) = alu::adc_bcd(
            registers.a,
            read_16bit_from_address(registers, bus, self.addressing_mode),
            registers.get_carry_flag(),
        );
        registers.a = result;
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_arithmetic(&registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(true, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}


#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test_adc_bin_8bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0x0000;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(true);
        bus.write(0x000001, 0x40);
        let instruction = ADC8BIN{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0x40);
        assert_eq!(registers.pc, 0x02);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn test_adc_bin_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0x0000;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(false);
        bus.write(0x000001, 0x00);
        bus.write(0x000002, 0x40);
        let instruction = ADC16BIN{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0x4000);
        assert_eq!(registers.pc, 0x03);
        assert_eq!(registers.cycles, 3);
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn test_adc_bcd_8bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0x0000;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(true);
        bus.write(0x000001, 0x40);
        let instruction = ADC8BCD{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0x40);
        assert_eq!(registers.pc, 0x02);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn test_adc_bcd_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0x0000;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(false);
        bus.write(0x000001, 0x00);
        bus.write(0x000002, 0x40);
        let instruction = ADC16BCD{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0x4000);
        assert_eq!(registers.pc, 0x03);
        assert_eq!(registers.cycles, 3);
        assert!(!registers.get_carry_flag());
    }
}
