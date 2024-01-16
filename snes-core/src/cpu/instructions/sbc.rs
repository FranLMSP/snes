use crate::{cpu::{bus::Bus, registers::Registers}, utils::{alu, addressing::AddressingMode}};

use crate::cpu::cycles;
use super::{CPUInstruction, read_write_common::{read_8bit_from_address, read_16bit_from_address}};
use super::decoder_common;

static INSTR_NAME: &str = "SBC";

pub struct SBC {
    pub addressing_mode: AddressingMode,
}

impl SBC {
    fn determine_instruction(&self, registers: &Registers) -> Box<dyn CPUInstruction> {
        let is_decimal_mode = registers.get_decimal_mode_flag();
        match registers.is_16bit_mode() {
            true => match is_decimal_mode {
                true => Box::new(SBC16BCD{addressing_mode: self.addressing_mode}),
                false => Box::new(SBC16BIN{addressing_mode: self.addressing_mode}),
            }
            false => match is_decimal_mode {
                true => Box::new(SBC8BCD{addressing_mode: self.addressing_mode}),
                false => Box::new(SBC8BIN{addressing_mode: self.addressing_mode}),
            }
        }
    }
}

impl CPUInstruction for SBC {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let instruction = self.determine_instruction(registers);
        instruction.execute(registers, bus);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        let instruction = self.determine_instruction(registers);
        instruction.mnemonic(registers, bus, opcode)
    }
}

pub struct SBC8BIN {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for SBC8BIN {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let (result, affected_flags) = alu::sbc_bin(
            registers.a as u8,
            read_8bit_from_address(registers, bus, self.addressing_mode),
            registers.get_carry_flag(),
        );
        registers.set_low_a(result);
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_arithmetic(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct SBC16BIN {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for SBC16BIN {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let (result, affected_flags) = alu::sbc_bin(
            registers.a,
            read_16bit_from_address(registers, bus, self.addressing_mode),
            registers.get_carry_flag(),
        );
        registers.a = result;
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_arithmetic(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(true, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct SBC8BCD {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for SBC8BCD {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let (result, affected_flags) = alu::sbc_bcd(
            registers.a as u8,
            read_8bit_from_address(registers, bus, self.addressing_mode),
            registers.get_carry_flag(),
        );
        registers.set_low_a(result);
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_arithmetic(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct SBC16BCD {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for SBC16BCD {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let (result, affected_flags) = alu::sbc_bcd(
            registers.a,
            read_16bit_from_address(registers, bus, self.addressing_mode),
            registers.get_carry_flag(),
        );
        registers.a = result;
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_arithmetic(registers, self.addressing_mode);
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
    fn test_sbc_bin_8bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0x0040;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(true);
        bus.write(0x000001, 0x40);
        let instruction = SBC8BIN{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0x00);
        assert_eq!(registers.pc, 0x02);
        assert_eq!(registers.cycles, 2);
        assert!(registers.get_zero_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn test_sbc_bin_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0x4000;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(false);
        bus.write(0x000001, 0x00);
        bus.write(0x000002, 0x40);
        let instruction = SBC16BIN{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0x0000);
        assert_eq!(registers.pc, 0x03);
        assert_eq!(registers.cycles, 3);
        assert!(registers.get_zero_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn test_sbc_bcd_8bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0x0049;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_16bit_mode(false);
        bus.write(0x000001, 0x48);
        let instruction = SBC8BCD{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0x00);
        assert_eq!(registers.pc, 0x02);
        assert_eq!(registers.cycles, 2);
        assert!(registers.get_zero_flag());
        assert!(registers.get_carry_flag());
    }

    #[test]
    fn test_sbc_bcd_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0x0049;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_16bit_mode(true);
        bus.write(0x000001, 0x48);
        bus.write(0x000002, 0x00);
        let instruction = SBC16BCD{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0x0000);
        assert_eq!(registers.pc, 0x03);
        assert_eq!(registers.cycles, 3);
        assert!(registers.get_zero_flag());
        assert!(registers.get_carry_flag());
    }
}
