use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::CPUInstruction;
use super::decoder_common;

static INSTR_NAME: &str = "STP";

pub struct STP {}

impl CPUInstruction for STP {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        registers.is_cpu_stopped = true;
        let (bytes, cycles) = cycles::increment_cycles_stp();
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
        registers.is_cpu_stopped = false;
        registers.pc = 0x0000;
        let instruction = STP{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pc, 0x0001);
        assert!(registers.is_cpu_stopped);
        assert_eq!(registers.cycles, 3);
    }
}
