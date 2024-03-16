use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::{CPUInstruction, pull_common};
use super::decoder_common;

static INSTR_NAME: &str = "RTI";

pub struct RTI {}

impl CPUInstruction for RTI {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        if registers.emulation_mode {
            registers.p = pull_common::do_pull(registers, bus, 1, false)[0];
            let pc_bytes = pull_common::do_pull(registers, bus, 2, false);
            registers.pc = (pc_bytes[0] as u16) | ((pc_bytes[1] as u16) << 8);
        } else {
            registers.p = pull_common::do_pull(registers, bus, 1, false)[0];
            let pc_bytes = pull_common::do_pull(registers, bus, 2, false);
            registers.pc = (pc_bytes[0] as u16) | ((pc_bytes[1] as u16) << 8);
            registers.pbr = pull_common::do_pull(registers, bus, 1, false)[0];
        }
        let (bytes, cycles) = cycles::increment_cycles_return_interrupt(registers.emulation_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, _registers: &Registers, _bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_single_byte_instr(opcode, INSTR_NAME)
    }
}
