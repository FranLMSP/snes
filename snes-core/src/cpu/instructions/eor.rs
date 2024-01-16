use crate::{cpu::{bus::Bus, registers::Registers}, utils::{alu, addressing::AddressingMode}};

use crate::cpu::cycles;
use super::{CPUInstruction, read_write_common::{read_8bit_from_address, read_16bit_from_address}};
use super::decoder_common;

static INSTR_NAME: &str = "EOR";

pub struct EOR {
    pub addressing_mode: AddressingMode,
}

impl EOR {
    fn determine_instruction(&self, registers: &Registers) -> Box<dyn CPUInstruction> {
        match registers.is_16bit_mode() {
            true => Box::new(EOR16{addressing_mode: self.addressing_mode}),
            false => Box::new(EOR8{addressing_mode: self.addressing_mode}),
        }
    }
}

impl CPUInstruction for EOR {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let instruction = self.determine_instruction(registers);
        instruction.execute(registers, bus);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        let instruction = self.determine_instruction(registers);
        instruction.mnemonic(registers, bus, opcode)
    }
}

pub struct EOR8 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for EOR8 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let value = read_8bit_from_address(registers, bus, self.addressing_mode);
        let (result, affected_flags) = alu::eor(registers.a as u8, value);
        registers.set_low_a(result);
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_bitwise(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct EOR16 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for EOR16 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let value = read_16bit_from_address(registers, bus, self.addressing_mode);
        let (result, affected_flags) = alu::eor(registers.a, value);
        registers.a = result;
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
    fn test_8bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.a   = 0x0F;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_16bit_mode(false);
        bus.write(0x000001, 0xF0);
        let instruction = EOR8{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0xFF);
        assert_eq!(registers.pc, 0x02);
        assert_eq!(registers.cycles, 2);
        assert!(registers.get_negative_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_and_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.a   = 0x0FFF;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_16bit_mode(false);
        bus.write(0x000002, 0xF0);
        bus.write(0x000001, 0x00);
        let instruction = EOR16{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0xFFFF);
        assert_eq!(registers.pc, 0x02);
        assert_eq!(registers.cycles, 2);
        assert!(registers.get_negative_flag());
        assert!(!registers.get_zero_flag());
    }
}
