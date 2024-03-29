use crate::cpu::{bus::Bus, registers::Registers};

use super::CPUInstruction;
use super::decoder_common;
use crate::cpu::cycles;

static INSTR_NAME: &str = "BRL";

pub struct BRL {}

impl CPUInstruction for BRL {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let label = bus.read(registers.get_pc_address() + 1) as u16 |
            ((bus.read(registers.get_pc_address() + 2) as u16) << 8);
        let is_negative = (label >> 15) != 0;
        if is_negative {
            let label = !label + 1;
            registers.pc = registers.pc.wrapping_sub(label);
        } else {
            registers.pc = registers.pc.wrapping_add(label);
        }
        let (bytes, cycles) = cycles::increment_cycles_branch_long();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_absolute_16bit(opcode, INSTR_NAME, registers, bus)
    }
}


#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test() {
        let instruction = BRL{};
        // test with positive nearlabel
        // branch always taken
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.pc  = 0x0001;
        registers.cycles        = 0;
        bus.write(0x02, 0b00000000);
        bus.write(0x03, 0b00001111);
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pc, 0x04 + 0b00001111_00000000);
        assert_eq!(registers.cycles, 4);
        // test with negative nearlabel and boundary cross
        registers.pc  = 0x00FC;
        registers.cycles        = 0;
        bus.write(0xFD, 0xFF); // write -1
        bus.write(0xFE, 0xFF); // write -1
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pc, 0xFE);
        assert_eq!(registers.cycles, 4);
    }
}
