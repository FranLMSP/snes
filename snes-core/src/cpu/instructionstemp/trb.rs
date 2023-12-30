use crate::cpu::cycles;
use crate::cpu::{bus::Bus, registers::Registers};
use crate::utils::addressing::AddressingMode;

use super::{CPUInstruction, Decode, read_8bit_from_address, write_8bit_to_address, read_16bit_from_address, write_16bit_to_address};
use super::decoder_common;

static INSTR_NAME: &'static str = "TRB";

pub struct TRB8 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for TRB8 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let value = read_8bit_from_address(registers, bus, self.addressing_mode);
        let result = (registers.a as u8) & value;
        write_8bit_to_address(registers, bus, self.addressing_mode, result);
        registers.set_zero_flag(result == 0);
        let (bytes, cycles) = cycles::increment_cycles_test(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for TRB8 {
    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct TRB16 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for TRB16 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let value = read_16bit_from_address(registers, bus, self.addressing_mode);
        let result = (registers.a) & value;
        write_16bit_to_address(registers, bus, self.addressing_mode, result);
        registers.set_zero_flag(result == 0);
        let (bytes, cycles) = cycles::increment_cycles_test(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for TRB16 {
    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(true, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}


#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test() {
    }
}
