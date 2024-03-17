use crate::cpu::cycles;
use crate::cpu::{bus::Bus, registers::Registers};

use super::CPUInstruction;
use super::decoder_common;

static INSTR_NAME: &str = "MVP";

pub struct MVP {}

impl CPUInstruction for MVP {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        let (bytes, _) = cycles::increment_cycles_move(1);
        registers.increment_pc(bytes);
        registers.is_moving = true;
        registers.is_move_next = false;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_move(opcode, INSTR_NAME, registers, bus)
    }
}
