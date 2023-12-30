use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use crate::utils::addressing::AddressingMode;
use super::{CPUInstruction, Decode, get_effective_address, push_common};
use super::decoder_common;

static INSTR_NAME: &'static str = "PER";

pub struct PER {}

impl CPUInstruction for PER {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let label = get_effective_address(registers, bus, AddressingMode::Absolute) as u16;
        let is_negative = (label>> 15) == 1;
        let (bytes, cycles) = cycles::increment_cycles_per();
        registers.increment_pc(bytes); registers.cycles += cycles;
        let address = match is_negative {
            true  => registers.pc.wrapping_sub(!label + 1),
            false => registers.pc.wrapping_add(label),
        };
        push_common::do_push(registers, bus, &[(address >> 8) as u8, address as u8]);
    }
}

impl Decode for PER {
    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_absolute(opcode, INSTR_NAME, registers, bus)
    }
}


#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.pc  = 0x0000;
        registers.sp  = 0x1FC;
        bus.write(0x000002, 0x00);
        bus.write(0x000001, 0x01);
        let instruction = PER{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(bus.read(0x1FC), 0x00);
        assert_eq!(bus.read(0x1FB), 0x04);
        assert_eq!(registers.pc, 0x0003);
        assert_eq!(registers.cycles, 6);
    }
}
