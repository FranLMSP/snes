use crate::cpu::{bus::Bus, registers::Registers};

use super::CPUInstruction;
use super::decoder_common;
use super::push_common;
use crate::cpu::cycles;

static INSTR_NAME: &str = "COP";

pub struct COP {}

impl CPUInstruction for COP {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        if !registers.emulation_mode {
            push_common::do_push(registers, bus, &[registers.pbr]);
        }
        let (bytes, cycles) = cycles::increment_cycles_brk(registers.emulation_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
        push_common::do_push(registers, bus, &[(registers.pc >> 8) as u8, registers.pc as u8]);
        push_common::do_push(registers, bus, &[registers.p]);
        registers.set_irq_disable_flag(true);
        registers.pbr = 0x00;
        if registers.emulation_mode {
            let vector = (bus.read(0x00FFF4) as u16) | ((bus.read(0x00FFF5) as u16) << 8);
            registers.pc = vector;
        } else {
            let vector = (bus.read(0x00FFE4) as u16) | ((bus.read(0x00FFE5) as u16) << 8);
            registers.pc = vector;
        }
        registers.set_decimal_mode_flag(false);
    }

    fn mnemonic(&self, _registers: &Registers, _bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_single_byte_instr(opcode, INSTR_NAME)
    }
}
