use crate::{cpu::{bus::Bus, registers::Registers}, utils::addressing::AddressingMode};

use crate::cpu::cycles;
use super::{CPUInstruction, Decode, write_8bit_to_address, write_16bit_to_address, read_8bit_from_address, read_16bit_from_address, dec_common};
use super::decoder_common;

static INSTR_NAME: &'static str = "DEC";

pub struct DEC8 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for DEC8 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let result = dec_common::do_dec(
            registers,
            read_8bit_from_address(registers, bus, self.addressing_mode),
        ) as u8;
        write_8bit_to_address(registers, bus, self.addressing_mode, result);
        let (bytes, cycles) = cycles::increment_cycles_inc_dec(&registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for DEC8 {
    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct DEC16 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for DEC16 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let result = dec_common::do_dec(
            registers,
            read_16bit_from_address(registers, bus, self.addressing_mode),
        ) as u16;
        write_16bit_to_address(registers, bus, self.addressing_mode, result);
        let (bytes, cycles) = cycles::increment_cycles_inc_dec(&registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for DEC16 {
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
        let instruction = DEC8{addressing_mode: AddressingMode::Accumulator};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_negative_flag());
        assert!(registers.get_zero_flag());
    }

    #[test]
    fn test_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.a   = 0x0001;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(true);
        let instruction = DEC16{addressing_mode: AddressingMode::Accumulator};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_negative_flag());
        assert!(registers.get_zero_flag());
    }
}
