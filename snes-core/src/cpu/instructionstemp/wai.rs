use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::{CPUInstruction, Decode};
use super::decoder_common;

static INSTR_NAME: &'static str = "WAI";

pub struct WAI {}

impl CPUInstruction for WAI {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        registers.is_cpu_waiting_interrupt = true;
        let (bytes, cycles) = cycles::increment_cycles_stp();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for WAI {
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
        registers.is_cpu_waiting_interrupt = false;
        registers.pc = 0x0000;
        let instruction = WAI{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.is_cpu_waiting_interrupt, true);
        assert_eq!(registers.cycles, 3);
    }
}
