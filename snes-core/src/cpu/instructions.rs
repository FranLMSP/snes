use super::cpu::CPU;
use crate::bus::Bus;
use crate::utils::addressing::{AddressingMode, IndexRegister};
use crate::utils::alu;
use crate::utils::num_trait::SnesNum;

impl CPU {
    fn get_8bit_from_address(&self, bus: &Bus, addressing_mode: AddressingMode) -> u8 {
        addressing_mode.value_8bit(
            bus,
            self.registers.get_pc_address(),
            self.registers.d,
            self.registers.sp,
            self.registers.x, self.registers.y
        )
    }

    fn get_16bit_from_address(&self, bus: &Bus, addressing_mode: AddressingMode) -> u16 {
        addressing_mode.value_16bit(
            bus,
            self.registers.get_pc_address(),
            self.registers.d,
            self.registers.sp,
            self.registers.x, self.registers.y
        )
    }

    fn adc(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        // if the M flag is set, perform 8 bit addition.
        // Otherwise, 16 bit addition
        let carry_flag = self.registers.get_carry_flag();
        let is_decimal_mode = self.registers.get_decimal_mode_flag();
        let is_16bit = self.registers.is_16bit_mode();
        let target = self.registers.a;
        if is_16bit {
            let value = self.get_16bit_from_address(bus, addressing_mode);
            let (result, affected_flags) = match is_decimal_mode {
                true => alu::adc_bcd(target, value, carry_flag),
                false => alu::adc_bin(target, value, carry_flag),
            };
            self.registers.a = result;
            self.registers.set_flags(&affected_flags);
        } else {
            let value = self.get_8bit_from_address(bus, addressing_mode);
            let (result, affected_flags) = match is_decimal_mode {
                true => alu::adc_bcd(target as u8, value, carry_flag),
                false => alu::adc_bin(target as u8, value, carry_flag),
            };
            self.registers.set_low_a(result as u8);
            self.registers.set_flags(&affected_flags);
        }
        self.increment_cycles_arithmetic(addressing_mode);
    }

    fn sbc(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        // if the M flag is set, perform 8 bit addition.
        // Otherwise, 16 bit addition
        let carry_flag = self.registers.get_carry_flag();
        let is_decimal_mode = self.registers.get_decimal_mode_flag();
        let is_16bit = self.registers.is_16bit_mode();
        let target = self.registers.a;
        if is_16bit {
            let value = self.get_16bit_from_address(bus, addressing_mode);
            let (result, affected_flags) = match is_decimal_mode {
                true => alu::sbc_bcd(target, value, carry_flag),
                false => alu::sbc_bin(target, value, carry_flag),
            };
            self.registers.a = result;
            self.registers.set_flags(&affected_flags);
        } else {
            let value = self.get_8bit_from_address(bus, addressing_mode);
            let (result, affected_flags) = match is_decimal_mode {
                true => alu::sbc_bcd(target as u8, value, carry_flag),
                false => alu::sbc_bin(target as u8, value, carry_flag),
            };
            self.registers.set_low_a(result as u8);
            self.registers.set_flags(&affected_flags);
        }
        self.increment_cycles_arithmetic(addressing_mode);
    }

    fn and(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        // if the M flag is set, perform 8 bit addition.
        // Otherwise, 16 bit addition
        let target = self.registers.a;
        if self.registers.is_16bit_mode() {
            let value = self.get_16bit_from_address(bus, addressing_mode);
            let (result, affected_flags) = alu::and(target, value);
            self.registers.a = result;
            self.registers.set_flags(&affected_flags);
        } else {
            let value = self.get_8bit_from_address(bus, addressing_mode);
            let (result, affected_flags) = alu::and(target as u8, value);
            self.registers.set_low_a(result);
            self.registers.set_flags(&affected_flags);

        }
        self.increment_cycles_bitwise(addressing_mode);
    }

    fn asl(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        // if the M flag is set, perform 8 bit addition.
        // Otherwise, 16 bit addition
        let target = match addressing_mode {
            AddressingMode::Accumulator => self.registers.a,
            _ => match self.registers.is_16bit_mode() {
                true => self.get_16bit_from_address(bus, addressing_mode),
                false => self.get_8bit_from_address(bus, addressing_mode) as u16,
            }
        };
        if self.registers.is_16bit_mode() {
            let (result, affected_flags) = alu::asl(target);
            self.registers.a = result;
            self.registers.set_flags(&affected_flags);
        } else {
            let (result, affected_flags) = alu::asl(target as u8);
            self.registers.set_low_a(result);
            self.registers.set_flags(&affected_flags);
        }
        self.increment_cycles_shift(addressing_mode);
    }

    fn do_bit<T: SnesNum>(&mut self, accumulator: T, value: T, addressing_mode: AddressingMode) {
        let (result, _) = alu::and(accumulator, value);
        // Immediate addressing affects only the zero flag
        match addressing_mode {
            AddressingMode::Immediate => self.registers.set_zero_flag(result.is_zero()),
            _ => {
                self.registers.set_zero_flag(result.is_zero());
                self.registers.set_negative_flag(value.is_negative());
                self.registers.set_overflow_flag(value.next_to_highest_bit());
            }
        };
    }

    fn bit(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        if self.registers.is_16bit_mode() {
            let value = self.get_16bit_from_address(bus, addressing_mode);
            self.do_bit(self.registers.a, value, addressing_mode);
        } else {
            let value = self.get_8bit_from_address(bus, addressing_mode);
            self.do_bit(self.registers.a as u8, value, addressing_mode);
        }
        self.increment_cycles_bit(addressing_mode);
    }

    pub fn do_branch(&mut self, bus: &Bus) -> bool {
        let nearlabel = bus.read(self.registers.get_pc_address());
        let is_negative = (nearlabel >> 7) != 0;
        let old_pc = self.registers.get_pc_address();
        if is_negative {
            let nearlabel = !nearlabel + 1;
            self.registers.decrement_pc(nearlabel as u16);
        } else {
            self.registers.increment_pc(nearlabel as u16);
        }
        let new_pc = self.registers.get_pc_address();
        let page_boundary_crossed = (old_pc & 0xFF00) != (new_pc & 0xFF00);
        return page_boundary_crossed
    }

    pub fn bcc(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if !self.registers.get_carry_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    pub fn bcs(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if self.registers.get_carry_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    pub fn beq(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if self.registers.get_zero_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    pub fn bne(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if !self.registers.get_zero_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    pub fn bmi(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if self.registers.get_negative_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    pub fn bpl(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if !self.registers.get_negative_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    pub fn bra(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        let page_boundary_crossed = self.do_branch(bus);
        self.increment_cycles_branch_taken(page_boundary_crossed);
    }

    pub fn brl(&mut self, bus: &Bus) {
        let label = bus.read(self.registers.get_pc_address()) as u16 |
            ((bus.read(self.registers.get_pc_address() + 1) as u16) << 8);
        let is_negative = (label >> 15) != 0;
        if is_negative {
            let label = !label + 1;
            self.registers.decrement_pc(label);
        } else {
            self.registers.increment_pc(label);
        }
        self.increment_cycles_branch_long();
    }

    pub fn bvc(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if !self.registers.get_overflow_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    pub fn bvs(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if self.registers.get_overflow_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    pub fn clc(&mut self) {
        self.registers.set_carry_flag(false);
        self.increment_cycles_clear();
    }

    pub fn cld(&mut self) {
        self.registers.set_decimal_mode_flag(false);
        self.increment_cycles_clear();
    }

    pub fn cli(&mut self) {
        self.registers.set_irq_disable_flag(false);
        self.increment_cycles_clear();
    }

    pub fn clv(&mut self) {
        self.registers.set_overflow_flag(false);
        self.increment_cycles_clear();
    }

    pub fn execute_opcode(&mut self, opcode: u8, bus: &Bus) {
        type A = AddressingMode;
        type I = IndexRegister;
        match opcode {
            // ADC
            0x69 => self.adc(bus, A::Immediate),
            0x6D => self.adc(bus, A::Absolute),
            0x6F => self.adc(bus, A::AbsoluteLong),
            0x65 => self.adc(bus, A::DirectPage),
            0x72 => self.adc(bus, A::DirectPageIndirect),
            0x67 => self.adc(bus, A::DirectPageIndirectLong),
            0x7D => self.adc(bus, A::AbsoluteIndexed(I::X)),
            0x7F => self.adc(bus, A::AbsoluteLongIndexed(I::X)),
            0x79 => self.adc(bus, A::AbsoluteIndexed(I::Y)),
            0x75 => self.adc(bus, A::DirectPageIndexed(I::X)),
            0x61 => self.adc(bus, A::DirectPageIndexedIndirect(I::X)),
            0x71 => self.adc(bus, A::DirectPageIndirectIndexed(I::Y)),
            0x77 => self.adc(bus, A::DirectPageIndirectLongIndexed(I::Y)),
            0x63 => self.adc(bus, A::StackRelative),
            0x73 => self.adc(bus, A::StackRelativeIndirectIndexed(I::Y)),
            // SBC
            0xE9 => self.sbc(bus, A::Immediate),
            0xED => self.sbc(bus, A::Absolute),
            0xEF => self.sbc(bus, A::AbsoluteLong),
            0xE5 => self.sbc(bus, A::DirectPage),
            0xF2 => self.sbc(bus, A::DirectPageIndirect),
            0xE7 => self.sbc(bus, A::DirectPageIndirectLong),
            0xFD => self.sbc(bus, A::AbsoluteIndexed(I::X)),
            0xFF => self.sbc(bus, A::AbsoluteLongIndexed(I::X)),
            0xF9 => self.sbc(bus, A::AbsoluteIndexed(I::Y)),
            0xF5 => self.sbc(bus, A::DirectPageIndexed(I::X)),
            0xE1 => self.sbc(bus, A::DirectPageIndexedIndirect(I::X)),
            0xF1 => self.sbc(bus, A::DirectPageIndirectIndexed(I::Y)),
            0xF7 => self.sbc(bus, A::DirectPageIndirectLongIndexed(I::Y)),
            0xE3 => self.sbc(bus, A::StackRelative),
            0xF3 => self.sbc(bus, A::StackRelativeIndirectIndexed(I::Y)),
            // AND
            0x29 => self.and(bus, A::Immediate),
            0x2D => self.and(bus, A::Absolute),
            0x2F => self.and(bus, A::AbsoluteLong),
            0x25 => self.and(bus, A::DirectPage),
            0x32 => self.and(bus, A::DirectPageIndirect),
            0x27 => self.and(bus, A::DirectPageIndirectLong),
            0x3D => self.and(bus, A::AbsoluteIndexed(I::X)),
            0x3F => self.and(bus, A::AbsoluteLongIndexed(I::X)),
            0x39 => self.and(bus, A::AbsoluteIndexed(I::Y)),
            0x35 => self.and(bus, A::DirectPageIndexed(I::X)),
            0x21 => self.and(bus, A::DirectPageIndexedIndirect(I::X)),
            0x31 => self.and(bus, A::DirectPageIndirectIndexed(I::Y)),
            0x37 => self.and(bus, A::DirectPageIndirectLongIndexed(I::Y)),
            0x23 => self.and(bus, A::StackRelative),
            0x33 => self.and(bus, A::StackRelativeIndirectIndexed(I::Y)),
            // ASL
            0x0A => self.asl(bus, A::Accumulator),
            0x0E => self.asl(bus, A::Absolute),
            0x06 => self.asl(bus, A::DirectPage),
            0x1E => self.asl(bus, A::AbsoluteIndexed(I::X)),
            0x16 => self.asl(bus, A::DirectPageIndexed(I::X)),
            // BCC
            0x90 => self.bcc(bus),
            // BCS
            0xB0 => self.bcs(bus),
            // BEQ
            0xF0 => self.beq(bus),
            // BNE
            0xD0 => self.bne(bus),
            // BMI
            0x30 => self.bmi(bus),
            // BPL
            0x10 => self.bpl(bus),
            // BRA
            0x80 => self.bra(bus),
            // BRL
            0x82 => self.brl(bus),
            // BVC
            0x50 => self.bvc(bus),
            // BVS
            0x70 => self.bvs(bus),
            // BIT
            0x89 => self.bit(bus, A::Immediate),
            0x2C => self.bit(bus, A::Absolute),
            0x24 => self.bit(bus, A::DirectPage),
            0x3C => self.bit(bus, A::AbsoluteIndexed(I::X)),
            0x34 => self.bit(bus, A::DirectPageIndexed(I::X)),
            // CLC
            0x18 => self.clc(),
            // CLD
            0xD8 => self.cld(),
            // CLI
            0x58 => self.cli(),
            // CLV
            0xB8 => self.clv(),
            _ => println!("Invalid opcode: {:02X}", opcode),
        }
    }
}

#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test_adc() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.a   = 0x0000;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_memory_select_flag(true);
        bus.write(0x000001, 0x40);
        cpu.adc(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.a, 0x40);
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_carry_flag());
    }

    #[test]
    fn test_sbc() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.a   = 0x0001;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_memory_select_flag(true);
        bus.write(0x000001, 1);
        cpu.sbc(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_carry_flag());
        assert!(cpu.registers.get_zero_flag());
    }

    #[test]
    fn test_and() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.a   = 0x0101;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_memory_select_flag(true);
        bus.write(0x000001, 0x01);
        bus.write(0x000002, 0x01);
        cpu.and(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.a, 0x0101);
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_carry_flag());
        assert!(!cpu.registers.get_zero_flag());
    }

    #[test]
    fn test_asl() {
        let mut cpu = CPU::new();
        let bus = Bus::new();
        cpu.registers.a   = 0b01010000_00000000;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_memory_select_flag(false);
        cpu.asl(&bus, AddressingMode::Accumulator);
        assert_eq!(cpu.registers.a, 0b10100000_00000000);
        assert_eq!(cpu.registers.pc, 0x01);
        assert_eq!(cpu.cycles, 4);
        assert!(!cpu.registers.get_carry_flag());
        assert!(!cpu.registers.get_zero_flag());
        assert!(cpu.registers.get_negative_flag());
    }

    #[test]
    fn test_bit() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.a   = 0b1111_0000;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.p   = 0x00;
        bus.write(0x000001, 0b0000_1111);
        cpu.registers.set_16bit_mode(false);
        cpu.bit(&bus, AddressingMode::Immediate);
        // Check that it only affects the zero flag on immediate mode
        assert_eq!(cpu.registers.a, 0b1111_0000); // Check that A is not altered
        assert_eq!(cpu.registers.p, 0b0010_0010); // Only zero flag was altered (bit 6 is memory select mode)
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        assert!(cpu.registers.get_zero_flag());

        cpu.registers.a   = 0b00110000_00000000;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.p   = 0x00;
        cpu.cycles        = 0;
        // Write absolute address
        bus.write(0x000001, 0x04);
        bus.write(0x000002, 0x00);
        // Write effective value of address
        bus.write(0x000004, 0x00);
        bus.write(0x000005, 0b1100_0000);
        cpu.registers.set_16bit_mode(true);
        cpu.bit(&bus, AddressingMode::Absolute);
        // Check that it only affects the zero flag on immediate mode
        assert_eq!(cpu.registers.a, 0b00110000_00000000); // Check that A is not altered
        assert_eq!(cpu.registers.p, 0b1100_0010);
        assert_eq!(cpu.registers.pc, 0x03);
        assert_eq!(cpu.cycles, 5);
        assert!(cpu.registers.get_zero_flag());
        assert!(cpu.registers.get_negative_flag());
        assert!(cpu.registers.get_overflow_flag());
    }

    #[test]
    fn test_clc() {
        let mut cpu = CPU::new();
        cpu.registers.set_carry_flag(true);
        cpu.registers.pc  = 0x0000;
        cpu.clc();
        assert!(!cpu.registers.get_carry_flag());
        assert_eq!(cpu.registers.pc, 1);
        assert_eq!(cpu.cycles, 2);
    }

    #[test]
    fn test_cld() {
        let mut cpu = CPU::new();
        cpu.registers.set_decimal_mode_flag(true);
        cpu.registers.pc  = 0x0000;
        cpu.cld();
        assert!(!cpu.registers.get_decimal_mode_flag());
        assert_eq!(cpu.registers.pc, 1);
        assert_eq!(cpu.cycles, 2);
    }

    #[test]
    fn test_cli() {
        let mut cpu = CPU::new();
        cpu.registers.set_irq_disable_flag(true);
        cpu.registers.pc  = 0x0000;
        cpu.cli();
        assert!(!cpu.registers.get_irq_disable_flag());
        assert_eq!(cpu.registers.pc, 1);
        assert_eq!(cpu.cycles, 2);
    }

    #[test]
    fn test_clv() {
        let mut cpu = CPU::new();
        cpu.registers.set_overflow_flag(true);
        cpu.registers.pc  = 0x0000;
        cpu.clv();
        assert!(!cpu.registers.get_overflow_flag());
        assert_eq!(cpu.registers.pc, 1);
        assert_eq!(cpu.cycles, 2);
    }

    #[test]
    fn test_bcc() {
        // test with positive nearlabel
        // branch not taken
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_carry_flag(true);
        bus.write(0x02, 0b00001111);
        cpu.bcc(&bus);
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        // branch taken
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_carry_flag(false);
        bus.write(0x02, 0b00001111);
        cpu.bcc(&bus);
        assert_eq!(cpu.registers.pc, 0x02 + 0b00001111);
        assert_eq!(cpu.cycles, 3);
        // test with negative nearlabel and boundary cross
        cpu.registers.pc  = 0x00FE;
        cpu.cycles        = 0;
        cpu.registers.set_carry_flag(false);
        bus.write(0x100, 0xFF); // write -1
        cpu.bcc(&bus);
        assert_eq!(cpu.registers.pc, 0xFF);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn test_bcs() {
        // test with positive nearlabel
        // branch not taken
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_carry_flag(false);
        bus.write(0x02, 0b00001111);
        cpu.bcs(&bus);
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        // branch taken
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_carry_flag(true);
        bus.write(0x02, 0b00001111);
        cpu.bcs(&bus);
        assert_eq!(cpu.registers.pc, 0x02 + 0b00001111);
        assert_eq!(cpu.cycles, 3);
        // test with negative nearlabel and boundary cross
        cpu.registers.pc  = 0x00FE;
        cpu.cycles        = 0;
        cpu.registers.set_carry_flag(true);
        bus.write(0x100, 0xFF); // write -1
        cpu.bcs(&bus);
        assert_eq!(cpu.registers.pc, 0xFF);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn test_beq() {
        // test with positive nearlabel
        // branch not taken
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_zero_flag(false);
        bus.write(0x02, 0b00001111);
        cpu.beq(&bus);
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        // branch taken
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_zero_flag(true);
        bus.write(0x02, 0b00001111);
        cpu.beq(&bus);
        assert_eq!(cpu.registers.pc, 0x02 + 0b00001111);
        assert_eq!(cpu.cycles, 3);
        // test with negative nearlabel and boundary cross
        cpu.registers.pc  = 0x00FE;
        cpu.cycles        = 0;
        cpu.registers.set_zero_flag(true);
        bus.write(0x100, 0xFF); // write -1
        cpu.beq(&bus);
        assert_eq!(cpu.registers.pc, 0xFF);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn test_bne() {
        // test with positive nearlabel
        // branch not taken
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_zero_flag(true);
        bus.write(0x02, 0b00001111);
        cpu.bne(&bus);
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        // branch taken
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_zero_flag(false);
        bus.write(0x02, 0b00001111);
        cpu.bne(&bus);
        assert_eq!(cpu.registers.pc, 0x02 + 0b00001111);
        assert_eq!(cpu.cycles, 3);
        // test with negative nearlabel and boundary cross
        cpu.registers.pc  = 0x00FE;
        cpu.cycles        = 0;
        cpu.registers.set_zero_flag(false);
        bus.write(0x100, 0xFF); // write -1
        cpu.bne(&bus);
        assert_eq!(cpu.registers.pc, 0xFF);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn test_bmi() {
        // test with positive nearlabel
        // branch not taken
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_negative_flag(false);
        bus.write(0x02, 0b00001111);
        cpu.bmi(&bus);
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        // branch taken
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_negative_flag(true);
        bus.write(0x02, 0b00001111);
        cpu.bmi(&bus);
        assert_eq!(cpu.registers.pc, 0x02 + 0b00001111);
        assert_eq!(cpu.cycles, 3);
        // test with negative nearlabel and boundary cross
        cpu.registers.pc  = 0x00FE;
        cpu.cycles        = 0;
        cpu.registers.set_negative_flag(true);
        bus.write(0x100, 0xFF); // write -1
        cpu.bmi(&bus);
        assert_eq!(cpu.registers.pc, 0xFF);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn test_bpl() {
        // test with positive nearlabel
        // branch not taken
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_negative_flag(true);
        bus.write(0x02, 0b00001111);
        cpu.bpl(&bus);
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        // branch taken
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_negative_flag(false);
        bus.write(0x02, 0b00001111);
        cpu.bpl(&bus);
        assert_eq!(cpu.registers.pc, 0x02 + 0b00001111);
        assert_eq!(cpu.cycles, 3);
        // test with negative nearlabel and boundary cross
        cpu.registers.pc  = 0x00FE;
        cpu.cycles        = 0;
        cpu.registers.set_negative_flag(false);
        bus.write(0x100, 0xFF); // write -1
        cpu.bpl(&bus);
        assert_eq!(cpu.registers.pc, 0xFF);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn test_bra() {
        // test with positive nearlabel
        // branch always taken
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        bus.write(0x02, 0b00001111);
        cpu.bra(&bus);
        assert_eq!(cpu.registers.pc, 0x02 + 0b00001111);
        assert_eq!(cpu.cycles, 3);
        // test with negative nearlabel and boundary cross
        cpu.registers.pc  = 0x00FE;
        cpu.cycles        = 0;
        bus.write(0x100, 0xFF); // write -1
        cpu.bra(&bus);
        assert_eq!(cpu.registers.pc, 0xFF);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn test_brl() {
        // test with positive nearlabel
        // branch always taken
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0001;
        cpu.cycles        = 0;
        bus.write(0x01, 0b00000000);
        bus.write(0x02, 0b00001111);
        cpu.brl(&bus);
        assert_eq!(cpu.registers.pc, 0x04 + 0b00001111_00000000);
        assert_eq!(cpu.cycles, 4);
        // test with negative nearlabel and boundary cross
        cpu.registers.pc  = 0x00FD;
        cpu.cycles        = 0;
        bus.write(0xFD, 0xFF); // write -1
        bus.write(0xFE, 0xFF); // write -1
        cpu.brl(&bus);
        assert_eq!(cpu.registers.pc, 0xFF);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn test_bvc() {
        // test with positive nearlabel
        // branch not taken
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_overflow_flag(true);
        bus.write(0x02, 0b00001111);
        cpu.bvc(&bus);
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        // branch taken
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_overflow_flag(false);
        bus.write(0x02, 0b00001111);
        cpu.bvc(&bus);
        assert_eq!(cpu.registers.pc, 0x02 + 0b00001111);
        assert_eq!(cpu.cycles, 3);
        // test with negative nearlabel and boundary cross
        cpu.registers.pc  = 0x00FE;
        cpu.cycles        = 0;
        cpu.registers.set_overflow_flag(false);
        bus.write(0x100, 0xFF); // write -1
        cpu.bvc(&bus);
        assert_eq!(cpu.registers.pc, 0xFF);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn test_bvs() {
        // test with positive nearlabel
        // branch not taken
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_overflow_flag(false);
        bus.write(0x02, 0b00001111);
        cpu.bvs(&bus);
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        // branch taken
        cpu.registers.pc  = 0x0000;
        cpu.cycles        = 0;
        cpu.registers.set_overflow_flag(true);
        bus.write(0x02, 0b00001111);
        cpu.bvs(&bus);
        assert_eq!(cpu.registers.pc, 0x02 + 0b00001111);
        assert_eq!(cpu.cycles, 3);
        // test with negative nearlabel and boundary cross
        cpu.registers.pc  = 0x00FE;
        cpu.cycles        = 0;
        cpu.registers.set_overflow_flag(true);
        bus.write(0x100, 0xFF); // write -1
        cpu.bvs(&bus);
        assert_eq!(cpu.registers.pc, 0xFF);
        assert_eq!(cpu.cycles, 4);
    }
}
