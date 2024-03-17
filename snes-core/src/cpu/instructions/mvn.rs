use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::CPUInstruction;
use super::decoder_common;

static INSTR_NAME: &str = "MVN";

pub struct MVN {}

impl CPUInstruction for MVN {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        let (bytes, _) = cycles::increment_cycles_move(1);
        registers.increment_pc(bytes);
        registers.is_moving = true;
        registers.is_move_next = true;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_move(opcode, INSTR_NAME, registers, bus)
    }
}
