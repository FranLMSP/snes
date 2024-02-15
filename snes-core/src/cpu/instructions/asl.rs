use crate::{cpu::{bus::Bus, registers::Registers}, utils::{alu, addressing::AddressingMode}};

use crate::cpu::cycles;
use super::{read_write_common::{read_16bit_from_address, read_8bit_from_address, write_16bit_to_address, write_8bit_to_address}, CPUInstruction};
use super::decoder_common;

static INSTR_NAME: &str = "ASL";

pub struct ASL {
    pub addressing_mode: AddressingMode,
}

impl ASL {
    fn determine_instruction(&self, registers: &Registers) -> Box<dyn CPUInstruction> {
        match registers.is_16bit_mode() {
            true => Box::new(ASL16{addressing_mode: self.addressing_mode}),
            false => Box::new(ASL8{addressing_mode: self.addressing_mode}),
        }
    }
}

impl CPUInstruction for ASL {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let instruction = self.determine_instruction(registers);
        instruction.execute(registers, bus);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        let instruction = self.determine_instruction(registers);
        instruction.mnemonic(registers, bus, opcode)
    }
}

pub struct ASL8 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for ASL8 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let (result, affected_flags) = alu::asl(
            read_8bit_from_address(registers, bus, self.addressing_mode),
        );
        write_8bit_to_address(registers, bus, self.addressing_mode, result);
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_bitwise(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct ASL16 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for ASL16 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let (result, affected_flags) = alu::asl(
            read_16bit_from_address(registers, bus, self.addressing_mode)
        );
        write_16bit_to_address(registers, bus, self.addressing_mode, result);
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_bitwise(registers, self.addressing_mode);
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
    fn test_asl_8bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0b01010000;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(true);
        let instruction = ASL8{addressing_mode: AddressingMode::Accumulator};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0b10100000);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_zero_flag());
        assert!(registers.get_negative_flag());
    }

    #[test]
    fn test_asl_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0b01010000_00000000;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(false);
        let instruction = ASL16{addressing_mode: AddressingMode::Accumulator};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0b10100000_00000000);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 3);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_zero_flag());
        assert!(registers.get_negative_flag());
    }
}
