use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use crate::utils::addressing::AddressingMode;
use super::read_write_common::get_effective_address;
use super::{CPUInstruction, push_common};
use super::decoder_common;

static INSTR_NAME: &str = "PEI";

pub struct PEI {}

impl CPUInstruction for PEI {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let address = get_effective_address(registers, bus, AddressingMode::DirectPageIndirect);
        push_common::do_push(registers, bus, &[(address >> 8) as u8, address as u8]);
        let (bytes, cycles) = cycles::increment_cycles_pei(registers);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_direct_page_indirect(opcode, INSTR_NAME, registers, bus)
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
        registers.d = 0x00;
        bus.write(0x000001, 0x02); // Direct page address
        bus.write(0x000002, 0xAA);
        bus.write(0x000003, 0xBB);
        let instruction = PEI{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(bus.read(0x1FC), 0xAA);
        assert_eq!(bus.read(0x1FB), 0xBB);
        assert_eq!(registers.pc, 0x0002);
        assert_eq!(registers.cycles, 6);
    }
}
