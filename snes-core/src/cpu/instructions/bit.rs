use crate::{cpu::{bus::Bus, registers::Registers}, utils::addressing::AddressingMode};

use crate::cpu::cycles;
use super::{CPUInstruction, bit_common, read_write_common::{read_8bit_from_address, read_16bit_from_address}};
use super::decoder_common;

static INSTR_NAME: &str = "BIT";

pub struct BIT {
    pub addressing_mode: AddressingMode,
}

impl BIT {
    fn determine_instruction(&self, registers: &Registers) -> Box<dyn CPUInstruction> {
        match registers.is_16bit_mode() {
            true => Box::new(BIT16{addressing_mode: self.addressing_mode}),
            false => Box::new(BIT8{addressing_mode: self.addressing_mode}),
        }
    }
}

impl CPUInstruction for BIT {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let instruction = self.determine_instruction(registers);
        instruction.execute(registers, bus);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        let instruction = self.determine_instruction(registers);
        instruction.mnemonic(registers, bus, opcode)
    }
}

pub struct BIT8 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for BIT8 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        bit_common::do_bit(
            registers,
            registers.a as u8,
            read_8bit_from_address(registers, bus, self.addressing_mode),
            self.addressing_mode,
        );
        let (bytes, cycles) = cycles::increment_cycles_bit(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct BIT16 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for BIT16 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        bit_common::do_bit(
            registers,
            registers.a,
            read_16bit_from_address(registers, bus, self.addressing_mode),
            self.addressing_mode,
        );
        let (bytes, cycles) = cycles::increment_cycles_bit(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(true, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}


#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test_8bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0b1111_0000;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.p   = 0x00;
        bus.write(0x000001, 0b0000_1111);
        registers.set_16bit_mode(false);
        let instruction = BIT8{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        // Check that it only affects the zero flag on immediate mode
        assert_eq!(registers.a, 0b1111_0000); // Check that A is not altered
        assert_eq!(registers.p, 0b0010_0010); // Only zero flag was altered (bit 6 is memory select mode)
        assert_eq!(registers.pc, 0x02);
        assert_eq!(registers.cycles, 2);
        assert!(registers.get_zero_flag());
    }

    #[test]
    fn test_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0b00110000_00000000;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.p   = 0x00;
        registers.cycles        = 0;
        // Write absolute address
        bus.write(0x000001, 0x04);
        bus.write(0x000002, 0x00);
        // Write effective value of address
        bus.write(0x000004, 0x00);
        bus.write(0x000005, 0b1100_0000);
        registers.set_16bit_mode(true);
        let instruction = BIT16{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        // Check that it only affects the zero flag on immediate mode
        assert_eq!(registers.a, 0b00110000_00000000); // Check that A is not altered
        assert_eq!(registers.p, 0b0000_0010);
        assert_eq!(registers.pc, 0x03);
        assert_eq!(registers.cycles, 3);
        assert!(registers.get_zero_flag());
    }
}
