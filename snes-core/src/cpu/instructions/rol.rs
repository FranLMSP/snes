use crate::{cpu::{bus::Bus, registers::Registers}, utils::{alu, addressing::AddressingMode}};

use crate::cpu::cycles;
use super::{CPUInstruction, read_write_common::{read_8bit_from_address, write_8bit_to_address, read_16bit_from_address, write_16bit_to_address}};
use super::decoder_common;

static INSTR_NAME: &'static str = "ROL";

pub struct ROL {
    pub addressing_mode: AddressingMode,
}

impl ROL {
    fn determine_instruction(&self, registers: &Registers) -> Box<dyn CPUInstruction> {
        match registers.is_16bit_mode() {
            true => Box::new(ROL16{addressing_mode: self.addressing_mode}),
            false => Box::new(ROL8{addressing_mode: self.addressing_mode}),
        }
    }
}

impl CPUInstruction for ROL {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let instruction = self.determine_instruction(registers);
        instruction.execute(registers, bus);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        let instruction = self.determine_instruction(registers);
        instruction.mnemonic(registers, bus, opcode)
    }
}

pub struct ROL8 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for ROL8 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let target = read_8bit_from_address(registers, bus, self.addressing_mode);
        let (result, affected_flags) = alu::rol(target, registers.get_carry_flag());
        write_8bit_to_address(registers, bus, self.addressing_mode, result);
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_shift(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct ROL16 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for ROL16 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let target = read_16bit_from_address(registers, bus, self.addressing_mode);
        let (result, affected_flags) = alu::rol(target, registers.get_carry_flag());
        write_16bit_to_address(registers, bus, self.addressing_mode, result);
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_shift(registers, self.addressing_mode);
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
    fn test_8bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.set_16bit_mode(false);
        registers.a  = 0b0100_0000;
        registers.pc  = 0x0000;
        let instruction = ROL8{addressing_mode: AddressingMode::Accumulator};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.get_negative_flag(), true);
        assert_eq!(registers.get_zero_flag(), false);
        assert_eq!(registers.a, 0b1000_0000);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.cycles, 2);
    }

    #[test]
    fn test_and_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.set_16bit_mode(true);
        registers.a  = 0b01000000_00000000;
        registers.pc  = 0x0000;
        let instruction = ROL16{addressing_mode: AddressingMode::Accumulator};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.get_negative_flag(), true);
        assert_eq!(registers.get_zero_flag(), false);
        assert_eq!(registers.a, 0b10000000_00000000);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.cycles, 4);
    }
}
