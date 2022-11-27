use super::cpu::CPU;
use crate::bus::Bus;
use crate::utils::addressing::{AddressingMode, IndexRegister};
use crate::utils::alu;
use crate::utils::num_trait::SnesNum;
use crate::common::flags::Flags;

impl CPU {
    fn get_8bit_from_address(&self, bus: &Bus, addressing_mode: AddressingMode) -> u8 {
        match addressing_mode {
            AddressingMode::Accumulator => self.registers.a as u8,
            _ => addressing_mode.value_8bit(
                bus,
                self.registers.get_pc_address(),
                self.registers.d,
                self.registers.sp,
                self.registers.x, self.registers.y
            )
        }
    }

    fn get_16bit_from_address(&self, bus: &Bus, addressing_mode: AddressingMode) -> u16 {
        match addressing_mode {
            AddressingMode::Accumulator => self.registers.a,
            _ => addressing_mode.value_16bit(
                bus,
                self.registers.get_pc_address(),
                self.registers.d,
                self.registers.sp,
                self.registers.x, self.registers.y
            )
        }
    }

    fn set_8bit_to_address(&mut self, bus: &mut Bus, addressing_mode: AddressingMode, value: u8) {
        match addressing_mode {
            AddressingMode::Accumulator => self.registers.set_low_a(value),
            _ => addressing_mode.store_8bit(
                bus,
                self.registers.get_pc_address(),
                self.registers.d,
                self.registers.sp,
                self.registers.x, self.registers.y,
                value,
            ),
        };
    }

    fn set_16bit_to_address(&mut self, bus: &mut Bus, addressing_mode: AddressingMode, value: u16) {
        match addressing_mode {
            AddressingMode::Accumulator => self.registers.a = value,
            _ => addressing_mode.store_16bit(
                bus,
                self.registers.get_pc_address(),
                self.registers.d,
                self.registers.sp,
                self.registers.x, self.registers.y,
                value,
            ),
        };
    }

    fn adc(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
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

    fn do_dec<T: SnesNum>(&mut self, target: T) -> T {
        let (result, affected_flags) = alu::sbc_bin(target, T::from_u32(1), false);
        for flag in affected_flags {
            match flag {
                Flags::Negative(_) | Flags::Zero(_) => self.registers.set_flags(&[flag]),
                _ => {},
            }
        }
        result
    }

    fn dec(&mut self, bus: &mut Bus, addressing_mode: AddressingMode) {
        if self.registers.is_16bit_mode() {
            let value = self.get_16bit_from_address(bus, addressing_mode);
            let result = self.do_dec(value).to_u32() as u16;
            self.set_16bit_to_address(bus, addressing_mode, result);
        } else {
            let value = self.get_8bit_from_address(bus, addressing_mode);
            let result = self.do_dec(value).to_u32() as u8;
            self.set_8bit_to_address(bus, addressing_mode, result);
        }
        self.increment_cycles_inc_dec(addressing_mode);
    }

    fn dex(&mut self) {
        if self.registers.is_16bit_index() {
            self.registers.x = self.do_dec(self.registers.x);
        } else {
            let result = self.do_dec(self.registers.x).to_u32() as u8;
            self.registers.set_low_x(result);
        }
        self.increment_cycles_inc_dec_index();
    }

    fn dey(&mut self) {
        if self.registers.is_16bit_index() {
            self.registers.y = self.do_dec(self.registers.y);
        } else {
            let result = self.do_dec(self.registers.y).to_u32() as u8;
            self.registers.set_low_y(result);
        }
        self.increment_cycles_inc_dec_index();
    }

    fn do_inc<T: SnesNum>(&mut self, target: T) -> T {
        let (result, affected_flags) = alu::adc_bin(target, T::from_u32(1), false);
        for flag in affected_flags {
            match flag {
                Flags::Negative(_) | Flags::Zero(_) => self.registers.set_flags(&[flag]),
                _ => {},
            }
        }
        result
    }

    fn inc(&mut self, bus: &mut Bus, addressing_mode: AddressingMode) {
        if self.registers.is_16bit_mode() {
            let value = self.get_16bit_from_address(bus, addressing_mode);
            let result = self.do_inc(value).to_u32() as u16;
            self.set_16bit_to_address(bus, addressing_mode, result);
        } else {
            let value = self.get_8bit_from_address(bus, addressing_mode);
            let result = self.do_inc(value).to_u32() as u8;
            self.set_8bit_to_address(bus, addressing_mode, result);
        }
        self.increment_cycles_inc_dec(addressing_mode);
    }

    fn inx(&mut self) {
        if self.registers.is_16bit_index() {
            self.registers.x = self.do_inc(self.registers.x);
        } else {
            let result = self.do_inc(self.registers.x).to_u32() as u8;
            self.registers.set_low_x(result);
        }
        self.increment_cycles_inc_dec_index();
    }

    fn iny(&mut self) {
        if self.registers.is_16bit_index() {
            self.registers.y = self.do_inc(self.registers.y);
        } else {
            let result = self.do_inc(self.registers.y).to_u32() as u8;
            self.registers.set_low_y(result);
        }
        self.increment_cycles_inc_dec_index();
    }

    fn do_comp<T: SnesNum>(&mut self, target: T, value: T) {
        let (_, affected_flags) = alu::sbc_bin(target, value, false);
        for flag in affected_flags {
            match flag {
                Flags::Overflow(_) => {},
                _ => self.registers.set_flags(&[flag]),
            }
        }
    }

    fn cmp(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        let is_16bit = self.registers.is_16bit_mode();
        let target = self.registers.a;
        if is_16bit {
            let value = self.get_16bit_from_address(bus, addressing_mode);
            self.do_comp(target, value);
        } else {
            let value = self.get_8bit_from_address(bus, addressing_mode);
            self.do_comp(target as u8, value);
        }
        self.increment_cycles_arithmetic(addressing_mode);
    }

    fn cpx(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        let is_16bit = self.registers.is_16bit_index();
        let target = self.registers.x;
        if is_16bit {
            let value = self.get_16bit_from_address(bus, addressing_mode);
            self.do_comp(target, value);
        } else {
            let value = self.get_8bit_from_address(bus, addressing_mode);
            self.do_comp(target as u8, value);
        }
        self.increment_cycles_comp_index(addressing_mode);
    }

    fn cpy(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        let is_16bit = self.registers.is_16bit_index();
        let target = self.registers.y;
        if is_16bit {
            let value = self.get_16bit_from_address(bus, addressing_mode);
            self.do_comp(target, value);
        } else {
            let value = self.get_8bit_from_address(bus, addressing_mode);
            self.do_comp(target as u8, value);
        }
        self.increment_cycles_comp_index(addressing_mode);
    }

    fn and(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
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

    fn eor(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        let target = self.registers.a;
        if self.registers.is_16bit_mode() {
            let value = self.get_16bit_from_address(bus, addressing_mode);
            let (result, affected_flags) = alu::eor(target, value);
            self.registers.a = result;
            self.registers.set_flags(&affected_flags);
        } else {
            let value = self.get_8bit_from_address(bus, addressing_mode);
            let (result, affected_flags) = alu::eor(target as u8, value);
            self.registers.set_low_a(result);
            self.registers.set_flags(&affected_flags);
        }
        self.increment_cycles_bitwise(addressing_mode);
    }

    fn asl(&mut self, bus: &mut Bus, addressing_mode: AddressingMode) {
        let target = match addressing_mode {
            AddressingMode::Accumulator => self.registers.a,
            _ => match self.registers.is_16bit_mode() {
                true => self.get_16bit_from_address(bus, addressing_mode),
                false => self.get_8bit_from_address(bus, addressing_mode) as u16,
            }
        };
        if self.registers.is_16bit_mode() {
            let (result, affected_flags) = alu::asl(target);
            self.set_16bit_to_address(bus, addressing_mode, result);
            self.registers.set_flags(&affected_flags);
        } else {
            let (result, affected_flags) = alu::asl(target as u8);
            self.set_8bit_to_address(bus, addressing_mode, result);
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

    pub fn execute_opcode(&mut self, opcode: u8, bus: &mut Bus) {
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
            // BRK
            0x00 => unimplemented!("BRK instruction not implemented yet"),
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
            // CMP
            0xC9 => self.cmp(bus, A::Immediate),
            0xCD => self.cmp(bus, A::Absolute),
            0xCF => self.cmp(bus, A::AbsoluteLong),
            0xC5 => self.cmp(bus, A::DirectPage),
            0xD2 => self.cmp(bus, A::DirectPageIndirect),
            0xC7 => self.cmp(bus, A::DirectPageIndirectLong),
            0xDD => self.cmp(bus, A::AbsoluteIndexed(I::X)),
            0xDF => self.cmp(bus, A::AbsoluteLongIndexed(I::X)),
            0xD9 => self.cmp(bus, A::AbsoluteIndexed(I::Y)),
            0xD5 => self.cmp(bus, A::DirectPageIndexed(I::X)),
            0xC1 => self.cmp(bus, A::DirectPageIndexedIndirect(I::X)),
            0xD1 => self.cmp(bus, A::DirectPageIndirectIndexed(I::Y)),
            0xD7 => self.cmp(bus, A::DirectPageIndirectLongIndexed(I::Y)),
            0xC3 => self.cmp(bus, A::StackRelative),
            0xD3 => self.cmp(bus, A::StackRelativeIndirectIndexed(I::Y)),
            // COP
            0x02 => unimplemented!("COP instruction not implemented yet"),
            // CPX
            0xE0 => self.cpx(bus, A::Immediate),
            0xEC => self.cpx(bus, A::Absolute),
            0xE4 => self.cpx(bus, A::DirectPage),
            // CPY
            0xC0 => self.cpy(bus, A::Immediate),
            0xCC => self.cpy(bus, A::Absolute),
            0xC4 => self.cpy(bus, A::DirectPage),
            // DEC
            0x3A => self.dec(bus, A::Accumulator),
            0xCE => self.dec(bus, A::Absolute),
            0xC6 => self.dec(bus, A::DirectPage),
            0xDE => self.dec(bus, A::AbsoluteIndexed(I::X)),
            0xD6 => self.dec(bus, A::DirectPageIndexed(I::X)),
            // DEX
            0xCA => self.dex(),
            // DEY
            0x88 => self.dey(),
            // EOR
            0x49 => self.eor(bus, A::Immediate),
            0x4D => self.eor(bus, A::Absolute),
            0x4F => self.eor(bus, A::AbsoluteLong),
            0x45 => self.eor(bus, A::DirectPage),
            0x52 => self.eor(bus, A::DirectPageIndirect),
            0x47 => self.eor(bus, A::DirectPageIndirectLong),
            0x5D => self.eor(bus, A::AbsoluteIndexed(I::X)),
            0x5F => self.eor(bus, A::AbsoluteLongIndexed(I::X)),
            0x59 => self.eor(bus, A::AbsoluteIndexed(I::Y)),
            0x55 => self.eor(bus, A::DirectPageIndexed(I::X)),
            0x41 => self.eor(bus, A::DirectPageIndexedIndirect(I::X)),
            0x51 => self.eor(bus, A::DirectPageIndirectIndexed(I::Y)),
            0x57 => self.eor(bus, A::DirectPageIndirectLongIndexed(I::Y)),
            0x43 => self.eor(bus, A::StackRelative),
            0x53 => self.eor(bus, A::StackRelativeIndirectIndexed(I::Y)),
            // INC
            0x1A => self.inc(bus, A::Accumulator),
            0xEE => self.inc(bus, A::Absolute),
            0xE6 => self.inc(bus, A::DirectPage),
            0xFE => self.inc(bus, A::AbsoluteIndexed(I::X)),
            0xF6 => self.inc(bus, A::DirectPageIndexed(I::X)),
            // INX
            0xE8 => self.inx(),
            // INY
            0xC8 => self.iny(),
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
    fn test_eor() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.a   = 0x0F;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_16bit_mode(false);
        bus.write(0x000001, 0xF0);
        cpu.eor(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.a, 0xFF);
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        assert!(cpu.registers.get_negative_flag());
        assert!(!cpu.registers.get_zero_flag());
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
        let mut bus = Bus::new();
        cpu.registers.a   = 0b01010000_00000000;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_memory_select_flag(false);
        cpu.asl(&mut bus, AddressingMode::Accumulator);
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

    #[test]
    fn test_cmp() {
        // CMP is basically an SBC instruction but it doesn't
        // store the result nor it affects the overflow flag
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.a   = 0x0001;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_memory_select_flag(true);
        bus.write(0x000001, 1);
        cpu.cmp(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.a, 0x0001); // check A is not affected
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_carry_flag());
        assert!(cpu.registers.get_zero_flag());

        // check overflow flag is not affected
        cpu.cycles = 0;
        cpu.registers.a   = 0x0050;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_16bit_mode(false);
        cpu.registers.set_overflow_flag(false);
        bus.write(0x000001, 0xB0);
        cpu.cmp(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.a, 0x0050); // check A is not affected
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        assert!(cpu.registers.get_carry_flag());
        assert!(!cpu.registers.get_zero_flag());
        assert!(!cpu.registers.get_overflow_flag());
    }

    #[test]
    fn test_cpx() {
        // CMP is basically an SBC instruction but it doesn't
        // store the result nor it affects the overflow flag
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.x   = 0x01;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_16bit_index(false);
        bus.write(0x000001, 1);
        cpu.cpx(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.x, 0x01); // check A is not affected
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_carry_flag());
        assert!(cpu.registers.get_zero_flag());

        // check overflow flag is not affected
        cpu.cycles = 0;
        cpu.registers.x   = 0x50;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_16bit_index(true);
        cpu.registers.set_overflow_flag(false);
        bus.write(0x000002, 0xB0);
        bus.write(0x000001, 0x00);
        cpu.cpx(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.x, 0x50); // check X is not affected
        assert_eq!(cpu.registers.pc, 0x03);
        assert_eq!(cpu.cycles, 3);
        assert!(cpu.registers.get_carry_flag());
        assert!(!cpu.registers.get_zero_flag());
        assert!(!cpu.registers.get_overflow_flag());
    }

    #[test]
    fn test_cpy() {
        // CMP is basically an SBC instruction but it doesn't
        // store the result nor it affects the overflow flag
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.y   = 0x01;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_16bit_index(false);
        bus.write(0x000001, 1);
        cpu.cpy(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.y, 0x01); // check A is not affected
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_carry_flag());
        assert!(cpu.registers.get_zero_flag());

        // check overflow flag is not affected
        cpu.cycles = 0;
        cpu.registers.y   = 0x50;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_16bit_index(true);
        cpu.registers.set_overflow_flag(false);
        bus.write(0x000002, 0xB0);
        bus.write(0x000001, 0x00);
        cpu.cpy(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.y, 0x50); // check X is not affected
        assert_eq!(cpu.registers.pc, 0x03);
        assert_eq!(cpu.cycles, 3);
        assert!(cpu.registers.get_carry_flag());
        assert!(!cpu.registers.get_zero_flag());
        assert!(!cpu.registers.get_overflow_flag());
    }

    #[test]
    fn test_dec() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.a   = 0x0001;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_memory_select_flag(true);
        cpu.dec(&mut bus, AddressingMode::Accumulator);
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.registers.pc, 0x01);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_negative_flag());
        assert!(cpu.registers.get_zero_flag());
    }

    #[test]
    fn test_dex() {
        let mut cpu = CPU::new();
        cpu.registers.x   = 0x0001;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.dex();
        assert_eq!(cpu.registers.x, 0);
        assert_eq!(cpu.registers.pc, 0x01);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_negative_flag());
        assert!(cpu.registers.get_zero_flag());
    }

    #[test]
    fn test_dey() {
        let mut cpu = CPU::new();
        cpu.registers.y   = 0x0001;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.dey();
        assert_eq!(cpu.registers.y, 0);
        assert_eq!(cpu.registers.pc, 0x01);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_negative_flag());
        assert!(cpu.registers.get_zero_flag());
    }

    #[test]
    fn test_inc() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.a   = 0x0001;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_memory_select_flag(true);
        cpu.inc(&mut bus, AddressingMode::Accumulator);
        assert_eq!(cpu.registers.a, 2);
        assert_eq!(cpu.registers.pc, 0x01);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_negative_flag());
        assert!(!cpu.registers.get_zero_flag());
    }

    #[test]
    fn test_inx() {
        let mut cpu = CPU::new();
        cpu.registers.x   = 0x0001;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.inx();
        assert_eq!(cpu.registers.x, 2);
        assert_eq!(cpu.registers.pc, 0x01);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_negative_flag());
        assert!(!cpu.registers.get_zero_flag());
    }

    #[test]
    fn test_iny() {
        let mut cpu = CPU::new();
        cpu.registers.y   = 0x0001;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.iny();
        assert_eq!(cpu.registers.y, 2);
        assert_eq!(cpu.registers.pc, 0x01);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_negative_flag());
        assert!(!cpu.registers.get_zero_flag());
    }
}
