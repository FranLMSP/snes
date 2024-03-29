use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::CPUInstruction;
use super::decoder_common;

static INSTR_NAME: &str = "XBA";

pub struct XBA {}

impl CPUInstruction for XBA {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        registers.a = (registers.a << 8) | (registers.a >> 8);
        registers.set_negative_flag(((registers.a as u8) >> 7) == 1);
        registers.set_zero_flag((registers.a as u8) == 0);
        let (bytes, cycles) = cycles::increment_cycles_xba();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, _registers: &Registers, _bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_single_byte_instr(opcode, INSTR_NAME)
    }
}


#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.pc = 0x0000;
        registers.a = 0x11FF;
        let instruction = XBA{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0xFF11);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.cycles, 3);
    }
}
