use crate::{cpu::{bus::Bus, registers::Registers}, utils::addressing::AddressingMode};

use crate::cpu::cycles;
use super::{CPUInstruction, dec_common, read_write_common::{read_8bit_from_address, write_8bit_to_address, read_16bit_from_address, write_16bit_to_address}};
use super::decoder_common;

static INSTR_NAME: &str = "INC";

pub struct INC {
    pub addressing_mode: AddressingMode,
}

impl INC {
    fn determine_instruction(&self, registers: &Registers) -> Box<dyn CPUInstruction> {
        match registers.is_16bit_mode() {
            true => Box::new(INC16{addressing_mode: self.addressing_mode}),
            false => Box::new(INC8{addressing_mode: self.addressing_mode}),
        }
    }
}

impl CPUInstruction for INC {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let instruction = self.determine_instruction(registers);
        instruction.execute(registers, bus);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        let instruction = self.determine_instruction(registers);
        instruction.mnemonic(registers, bus, opcode)
    }
}

pub struct INC8 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for INC8 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let result = dec_common::do_inc(
            registers,
            read_8bit_from_address(registers, bus, self.addressing_mode),
        ) as u8;
        write_8bit_to_address(registers, bus, self.addressing_mode, result);
        let (bytes, cycles) = cycles::increment_cycles_inc_dec(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct INC16 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for INC16 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let result = dec_common::do_inc(
            registers,
            read_16bit_from_address(registers, bus, self.addressing_mode),
        ) as u16;
        write_16bit_to_address(registers, bus, self.addressing_mode, result);
        let (bytes, cycles) = cycles::increment_cycles_inc_dec(registers, self.addressing_mode);
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
        registers.a   = 0x0001;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(true);
        let instruction = INC8{addressing_mode: AddressingMode::Accumulator};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 2);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_negative_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.a   = 0x0001;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(false);
        let instruction = INC16{addressing_mode: AddressingMode::Accumulator};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 2);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_negative_flag());
        assert!(!registers.get_zero_flag());
    }
}
