use crate::{cpu::{bus::Bus, registers::Registers}, utils::addressing::AddressingMode};

use crate::cpu::cycles;
use super::{CPUInstruction, read_write_common::{read_8bit_from_address, read_16bit_from_address}};
use super::decoder_common;
use super::comp_common;

static INSTR_NAME: &str = "CPY";

pub struct CPY {
    pub addressing_mode: AddressingMode,
}

impl CPY {
    fn determine_instruction(&self, registers: &Registers) -> Box<dyn CPUInstruction> {
        match registers.is_16bit_index() {
            true => Box::new(CPY16{addressing_mode: self.addressing_mode}),
            false => Box::new(CPY8{addressing_mode: self.addressing_mode}),
        }
    }
}

impl CPUInstruction for CPY {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let instruction = self.determine_instruction(registers);
        instruction.execute(registers, bus);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        let instruction = self.determine_instruction(registers);
        instruction.mnemonic(registers, bus, opcode)
    }
}

pub struct CPY8 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for CPY8 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        comp_common::do_comp(
            registers,
            registers.y as u8,
            read_8bit_from_address(registers, bus, self.addressing_mode),
        );
        let (bytes, cycles) = cycles::increment_cycles_comp_index(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct CPY16 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for CPY16 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        comp_common::do_comp(
            registers,
            registers.y,
            read_16bit_from_address(registers, bus, self.addressing_mode),
        );
        let (bytes, cycles) = cycles::increment_cycles_comp_index(registers, self.addressing_mode);
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
        // CMP is basically an SBC instruction but it doesn't
        // store the result nor it affects the overflow flag
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0x10;
        registers.y   = 0x01;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_16bit_index(false);
        bus.write(0x000001, 1);
        let instruction = CPY8{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.y, 0x01);
        assert_eq!(registers.pc, 0x02);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_carry_flag());
        assert!(registers.get_zero_flag());
    }

    #[test]
    fn test_and_16bit() {
        // check overflow flag is not affected
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.cycles = 0;
        registers.a   = 0x10;
        registers.y   = 0x50;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.emulation_mode = false;
        registers.set_16bit_index(true);
        registers.set_overflow_flag(false);
        bus.write(0x000002, 0xB0);
        bus.write(0x000001, 0x00);
        let instruction = CPY16{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.y, 0x50); // check Y is not affected
        assert_eq!(registers.pc, 0x03);
        assert_eq!(registers.cycles, 3);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_overflow_flag());
    }
}
