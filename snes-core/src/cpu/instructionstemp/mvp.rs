use crate::cpu::{bus::Bus, registers::Registers};

use super::{CPUInstruction, move_common};
use super::decoder_common;

static INSTR_NAME: &'static str = "MVP";

pub struct MVP {}

impl CPUInstruction for MVP {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        move_common::do_move(registers, bus, false);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_move(opcode, INSTR_NAME, registers, bus)
    }
}


#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test() {
    }
}
