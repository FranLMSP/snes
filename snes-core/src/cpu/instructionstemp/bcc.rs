use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::{CPUInstruction, Decode};
use super::decoder_common;
use super::branch_common;

static INSTR_NAME: &'static str = "BCC";

pub struct BCC {}

impl CPUInstruction for BCC {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let nearlabel = bus.read(registers.get_pc_address().wrapping_add(1));
        let (bytes, cycles) = cycles::increment_cycles_branch();
        registers.increment_pc(bytes); registers.cycles += cycles;
        if !registers.get_carry_flag() {
            let page_boundary_crossed = branch_common::do_branch(nearlabel, registers);
            let (bytes, cycles) = cycles::increment_cycles_branch_taken(page_boundary_crossed);
            registers.increment_pc(bytes); registers.cycles += cycles;
        }
    }
}

impl Decode for BCC {
    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_branch_nearlabel(opcode, INSTR_NAME, registers, bus)
    }
}


#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test_bcc() {
        let instruction = BCC{};
        // test with positive nearlabel
        // branch not taken
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.pc = 0x0000;
        registers.cycles = 0;
        registers.set_carry_flag(true);
        bus.write(0x02, 0b00001111);
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pc, 0x02);
        assert_eq!(registers.cycles, 2);
        // branch taken
        registers.pc  = 0x0000;
        registers.cycles        = 0;
        registers.set_carry_flag(false);
        bus.write(0x01, 0b00001111);
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pc, 0x02 + 0b00001111);
        assert_eq!(registers.cycles, 3);
        // test with negative nearlabel and boundary cross
        registers.pc  = 0x0100;
        registers.cycles        = 0;
        registers.set_carry_flag(false);
        bus.write(0x101, 0xFB); // write -5
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pc, 0xFD);
        assert_eq!(registers.cycles, 4);
    }
}
