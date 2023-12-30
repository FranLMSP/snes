use crate::cpu::cycles;
use crate::cpu::{bus::Bus, registers::Registers};

use super::{CPUInstruction, Decode};
use super::decoder_common;

static INSTR_NAME: &'static str = "TXY";

pub struct TXY8 {}

impl CPUInstruction for TXY8 {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        let result = registers.x as u8;
        registers.set_low_y(result);
        registers.set_negative_flag((result >> 7) == 1);
        registers.set_zero_flag(result == 0);
        let (bytes, cycles) = cycles::increment_cycles_transfer();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for TXY8 {
    fn mnemonic(&self, _registers: &Registers, _bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_single_byte_instr(opcode, INSTR_NAME)
    }
}

pub struct TXY16 {}

impl CPUInstruction for TXY16 {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        let result = registers.x;
        registers.y = result;
        registers.set_negative_flag((result >> 15) == 1);
        registers.set_zero_flag(result == 0);
        let (bytes, cycles) = cycles::increment_cycles_transfer();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for TXY16 {
    fn mnemonic(&self, _registers: &Registers, _bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_single_byte_instr(opcode, INSTR_NAME)
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
        registers.pc = 0x0000;
        registers.y = 0x0000;
        registers.x = 0xF0F0;
        registers.set_16bit_index(false);
        let instruction = TXY8{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.y, 0x00F0);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.cycles, 2);
    }

    #[test]
    fn test_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.pc = 0x0000;
        registers.y = 0x0000;
        registers.x = 0xF0F0;
        registers.set_16bit_index(true);
        let instruction = TXY16{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.y, 0xF0F0);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.cycles, 2);
    }
}
