use crate::cpu::{bus::Bus, registers::Registers};

use super::{CPUInstruction, move_common};
use super::decoder_common;

static INSTR_NAME: &str = "MVN";

pub struct MVN {}

impl CPUInstruction for MVN {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        move_common::do_move(registers, bus, true);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_move(opcode, INSTR_NAME, registers, bus)
    }
}
