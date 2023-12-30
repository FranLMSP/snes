use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::{CPUInstruction, Decode, pull_common};
use super::decoder_common;

static INSTR_NAME: &'static str = "RTI";

pub struct RTI {}

impl CPUInstruction for RTI {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        registers.p = pull_common::do_pull(registers, bus, 1)[0];
        let pc_bytes = pull_common::do_pull(registers, bus, 2);
        registers.pc = (pc_bytes[0] as u16) | ((pc_bytes[1] as u16) << 8);
        if !registers.emulation_mode {
            registers.pbr = pull_common::do_pull(registers, bus, 1)[0];
        }
        let (bytes, cycles) = cycles::increment_cycles_return_interrupt(registers.emulation_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for RTI {
    fn mnemonic(&self, _registers: &Registers, _bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_single_byte_instr(opcode, INSTR_NAME)
    }
}


#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test() {
    }
}
