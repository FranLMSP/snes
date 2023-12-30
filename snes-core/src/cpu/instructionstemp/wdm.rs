use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::{CPUInstruction, Decode};
use super::decoder_common;

static INSTR_NAME: &'static str = "WDM";

pub struct WDM {}

impl CPUInstruction for WDM {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        let (bytes, cycles) = cycles::increment_cycles_wdm();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for WDM {
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
        let instruction = WDM{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pc, 0x0002);
        assert_eq!(registers.cycles, 2);
    }
}
