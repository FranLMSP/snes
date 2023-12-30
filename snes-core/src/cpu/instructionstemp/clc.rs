use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::CPUInstruction;
use super::decoder_common;

static INSTR_NAME: &'static str = "CLC";

pub struct CLC {}

impl CPUInstruction for CLC {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        registers.set_carry_flag(false);
        let (bytes, cycles) = cycles::increment_cycles_clear();
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
        registers.set_carry_flag(true);
        registers.pc  = 0x0000;
        let instruction = CLC{};
        instruction.execute(&mut registers, &mut bus);
        assert!(!registers.get_carry_flag());
        assert_eq!(registers.pc, 1);
        assert_eq!(registers.cycles, 2);
    }
}
