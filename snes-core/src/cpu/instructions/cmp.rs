use crate::{cpu::{bus::Bus, registers::Registers}, utils::addressing::AddressingMode};

use crate::cpu::cycles;
use super::{CPUInstruction, read_write_common::{read_8bit_from_address, read_16bit_from_address}};
use super::decoder_common;
use super::comp_common;

static INSTR_NAME: &str = "CMP";

pub struct CMP {
    pub addressing_mode: AddressingMode,
}

impl CMP {
    fn determine_instruction(&self, registers: &Registers) -> Box<dyn CPUInstruction> {
        match registers.is_16bit_mode() {
            true => Box::new(CMP16{addressing_mode: self.addressing_mode}),
            false => Box::new(CMP8{addressing_mode: self.addressing_mode}),
        }
    }
}

impl CPUInstruction for CMP {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let instruction = self.determine_instruction(registers);
        instruction.execute(registers, bus);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        let instruction = self.determine_instruction(registers);
        instruction.mnemonic(registers, bus, opcode)
    }
}

pub struct CMP8 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for CMP8 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        comp_common::do_comp(
            registers,
            registers.a as u8,
            read_8bit_from_address(registers, bus, self.addressing_mode),
        );
        let (bytes, cycles) = cycles::increment_cycles_arithmetic(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct CMP16 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for CMP16 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        comp_common::do_comp(
            registers,
            registers.a,
            read_16bit_from_address(registers, bus, self.addressing_mode),
        );
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
    fn test_8bit() {
        // CMP is basically an SBC instruction but it doesn't
        // store the result nor it affects the overflow flag
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.a   = 0x0001;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(true);
        bus.write(0x000001, 1);
        let instruction = CMP8{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0x0001); // check A is not affected
        assert_eq!(registers.pc, 0x02);
        assert_eq!(registers.cycles, 2);
        assert!(registers.get_carry_flag());
        assert!(registers.get_zero_flag());
    }

    #[test]
    fn test_and_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.a   = 0x0050;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_16bit_mode(false);
        registers.set_overflow_flag(false);
        registers.set_carry_flag(true);
        bus.write(0x000001, 0xB0);
        let instruction = CMP16{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0x0050); // check A is not affected
        assert_eq!(registers.pc, 0x02);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_overflow_flag());
    }
}
