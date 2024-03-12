use crate::cpu::{bus::Bus, registers::Registers};

use super::CPUInstruction;
use super::decoder_common;
use super::push_common;
use crate::cpu::cycles;

static INSTR_NAME: &str = "BRK";

pub struct BRK {}

impl CPUInstruction for BRK {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        push_common::do_push(registers, bus, &[registers.pbr]);
        let (bytes, cycles) = cycles::increment_cycles_brk(registers.emulation_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
        push_common::do_push(registers, bus, &[(registers.pc >> 8) as u8, registers.pc as u8]);
        push_common::do_push(registers, bus, &[registers.p]);
        registers.set_irq_disable_flag(true);
        registers.pbr = 0x00;
        let vector = (bus.read(0x00FFE6) as u16) | ((bus.read(0x00FFE7) as u16) << 8);
        registers.pc = vector;
        registers.set_decimal_mode_flag(false);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_8bit_immediate(opcode, INSTR_NAME, registers, bus)
    }
}
