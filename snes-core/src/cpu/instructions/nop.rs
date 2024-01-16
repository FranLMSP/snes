use crate::cpu::cycles;
use crate::cpu::{bus::Bus, registers::Registers};

use super::CPUInstruction;
use super::decoder_common;

static INSTR_NAME: &str = "NOP";

pub struct NOP {}

impl CPUInstruction for NOP {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        let (bytes, cycles) = cycles::increment_cycles_nop();
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
        registers.pc  = 0x0000;
        let instruction = NOP{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 2);
    }
}
