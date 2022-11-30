use super::cpu::CPU;
use crate::bus::Bus;
use crate::utils::addressing::{AddressingMode, IndexRegister};
use crate::utils::alu;
use crate::utils::num_trait::SnesNum;
use crate::common::flags::Flags;

impl CPU {
    fn get_effective_address(&self, bus: &Bus, addressing_mode: AddressingMode) -> u32 {
        addressing_mode.effective_address(
            bus,
            self.registers.get_pc_address(),
            self.registers.d,
            self.registers.sp,
            self.registers.x, self.registers.y
        )
    }

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

    fn ora(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        let target = self.registers.a;
        if self.registers.is_16bit_mode() {
            let value = self.get_16bit_from_address(bus, addressing_mode);
            let (result, affected_flags) = alu::ora(target, value);
            self.registers.a = result;
            self.registers.set_flags(&affected_flags);
        } else {
            let value = self.get_8bit_from_address(bus, addressing_mode);
            let (result, affected_flags) = alu::ora(target as u8, value);
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

    fn lsr(&mut self, bus: &mut Bus, addressing_mode: AddressingMode) {
        let target = match addressing_mode {
            AddressingMode::Accumulator => self.registers.a,
            _ => match self.registers.is_16bit_mode() {
                true => self.get_16bit_from_address(bus, addressing_mode),
                false => self.get_8bit_from_address(bus, addressing_mode) as u16,
            }
        };
        if self.registers.is_16bit_mode() {
            let (result, affected_flags) = alu::lsr(target);
            self.set_16bit_to_address(bus, addressing_mode, result);
            self.registers.set_flags(&affected_flags);
        } else {
            let (result, affected_flags) = alu::lsr(target as u8);
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

    fn do_branch(&mut self, bus: &Bus) -> bool {
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

    fn bcc(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if !self.registers.get_carry_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    fn bcs(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if self.registers.get_carry_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    fn beq(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if self.registers.get_zero_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    fn bne(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if !self.registers.get_zero_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    fn bmi(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if self.registers.get_negative_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    fn bpl(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if !self.registers.get_negative_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    fn bra(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        let page_boundary_crossed = self.do_branch(bus);
        self.increment_cycles_branch_taken(page_boundary_crossed);
    }

    fn brl(&mut self, bus: &Bus) {
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

    fn bvc(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if !self.registers.get_overflow_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    fn bvs(&mut self, bus: &Bus) {
        self.increment_cycles_branch();
        if self.registers.get_overflow_flag() {
            let page_boundary_crossed = self.do_branch(bus);
            self.increment_cycles_branch_taken(page_boundary_crossed);
        }
    }

    fn clc(&mut self) {
        self.registers.set_carry_flag(false);
        self.increment_cycles_clear();
    }

    fn cld(&mut self) {
        self.registers.set_decimal_mode_flag(false);
        self.increment_cycles_clear();
    }

    fn cli(&mut self) {
        self.registers.set_irq_disable_flag(false);
        self.increment_cycles_clear();
    }

    fn clv(&mut self) {
        self.registers.set_overflow_flag(false);
        self.increment_cycles_clear();
    }

    fn nop(&mut self) {
        self.increment_cycles_nop();
    }

    fn jmp(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        let effective_address = self.get_effective_address(bus, addressing_mode);
        let is_long = match addressing_mode {
            AddressingMode::AbsoluteLong |
            AddressingMode::AbsoluteIndirectLong => true,
            _  => false,
        };
        self.registers.pc = effective_address as u16;
        if is_long {
            self.registers.pbr = (effective_address >> 16) as u8;
        }
        self.increment_cycles_jmp(addressing_mode);
    }

    fn do_push(&mut self, bus: &mut Bus, bytes: &[u8]) {
        for byte in bytes {
            let address = self.registers.sp as u32;
            bus.write(address, *byte);
            self.registers.decrement_sp(1);
        }
    }

    fn pea(&mut self, bus: &mut Bus) {
        let address = self.get_effective_address(bus, AddressingMode::Absolute);
        self.do_push(bus, &[(address >> 8) as u8, address as u8]);
        self.increment_cycles_pea();
    }

    fn pei(&mut self, bus: &mut Bus) {
        let address = self.get_effective_address(bus, AddressingMode::DirectPageIndirect);
        self.do_push(bus, &[(address >> 8) as u8, address as u8]);
        self.increment_cycles_pei();
    }

    fn per(&mut self, bus: &mut Bus) {
        let label = self.get_effective_address(bus, AddressingMode::Absolute) as u16;
        let is_negative = (label>> 15) == 1;
        self.increment_cycles_per();
        let address = match is_negative {
            true => self.registers.pc.wrapping_sub(!label + 1),
            false=> self.registers.pc.wrapping_add(label),
        };
        self.do_push(bus, &[(address >> 8) as u8, address as u8]);
    }

    fn pha(&mut self, bus: &mut Bus) {
        let value = self.registers.a;
        if self.registers.is_16bit_mode() {
            self.do_push(bus, &[(value >> 8) as u8, value as u8]);
        } else {
            self.do_push(bus, &[value as u8]);
        }
        self.increment_cycles_pha();
    }

    fn phb(&mut self, bus: &mut Bus) {
        self.do_push(bus, &[self.registers.dbr]);
        self.increment_cycles_phb();
    }

    fn phd(&mut self, bus: &mut Bus) {
        let value = self.registers.d;
        self.do_push(bus, &[(value >> 8) as u8, value as u8]);
        self.increment_cycles_phd();
    }

    fn phk(&mut self, bus: &mut Bus) {
        self.do_push(bus, &[self.registers.pbr]);
        self.increment_cycles_phk();
    }

    fn php(&mut self, bus: &mut Bus) {
        self.do_push(bus, &[self.registers.p]);
        self.increment_cycles_php();
    }

    fn phx(&mut self, bus: &mut Bus) {
        let value = self.registers.x;
        if self.registers.is_16bit_index() {
            self.do_push(bus, &[(value >> 8) as u8, value as u8]);
        } else {
            self.do_push(bus, &[value as u8]);
        }
        self.increment_cycles_push_index();
    }

    fn phy(&mut self, bus: &mut Bus) {
        let value = self.registers.y;
        if self.registers.is_16bit_index() {
            self.do_push(bus, &[(value >> 8) as u8, value as u8]);
        } else {
            self.do_push(bus, &[value as u8]);
        }
        self.increment_cycles_push_index();
    }

    fn jsr(&mut self, bus: &mut Bus, addressing_mode: AddressingMode) {
        let effective_address = self.get_effective_address(bus, addressing_mode);
        let is_long = match addressing_mode {
            AddressingMode::AbsoluteLong |
            AddressingMode::AbsoluteIndirectLong => true,
            _  => false,
        };
        // We need to push the *next* instruction onto the stack
        self.increment_cycles_jsr(addressing_mode);
        let value = self.registers.get_pc_address();
        if is_long {
            self.do_push(bus, &[
                (value >> 16) as u8,
                (value >> 8) as u8,
                value as u8,
            ]);
        } else {
            self.do_push(bus, &[
                (value >> 8) as u8,
                value as u8,
            ]);
        }
        self.registers.pc = effective_address as u16;
        if is_long {
            self.registers.pbr = (effective_address >> 16) as u8;
        }
    }

    fn lda(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        if self.registers.is_16bit_mode() {
            let value = self.get_16bit_from_address(bus, addressing_mode);
            self.registers.a = value;
            self.registers.set_flags(&[
                Flags::Negative(value >> 15 == 1),
                Flags::Zero(value == 0),
            ]);
        } else {
            let value = self.get_8bit_from_address(bus, addressing_mode);
            self.registers.set_flags(&[
                Flags::Negative(value >> 7 == 1),
                Flags::Zero(value == 0),
            ]);
            self.registers.set_low_a(value);
            self.do_bit(self.registers.a as u8, value, addressing_mode);
        }
        self.increment_cycles_lda(addressing_mode);
    }

    fn do_ld_index(&mut self, bus: &Bus, index: IndexRegister, addressing_mode: AddressingMode) {
        if self.registers.is_16bit_index() {
            let value = self.get_16bit_from_address(bus, addressing_mode);
            match index {
                IndexRegister::X => self.registers.x = value,
                IndexRegister::Y => self.registers.y = value,
            }
            self.registers.set_flags(&[
                Flags::Negative(value >> 7 == 1),
                Flags::Zero(value == 0),
            ]);
        } else {
            let value = self.get_8bit_from_address(bus, addressing_mode);
            match index {
                IndexRegister::X => self.registers.set_low_x(value),
                IndexRegister::Y => self.registers.set_low_y(value),
            }
            self.registers.set_flags(&[
                Flags::Negative(value >> 7 == 1),
                Flags::Zero(value == 0),
            ]);
        }
        self.increment_cycles_ld_index(addressing_mode);
    }

    fn ldx(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        self.do_ld_index(bus, IndexRegister::X, addressing_mode);
    }

    fn ldy(&mut self, bus: &Bus, addressing_mode: AddressingMode) {
        self.do_ld_index(bus, IndexRegister::Y, addressing_mode);
    }

    fn do_pull(&mut self, bus: &Bus, count: usize) -> Vec<u8> {
        let mut bytes = vec![];
        let mut is_zero = true;
        for _ in 0..count {
            self.registers.increment_sp(1);
            let byte = bus.read(self.registers.sp as u32);
            if byte != 0 {
                is_zero = false;
            }
            bytes.push(byte);
        }
        self.registers.set_zero_flag(is_zero);
        if bytes.len() > 0 {
            // Low byte is pulled first, so we need to check
            // for the last byte that we pull
            self.registers.set_negative_flag((bytes[bytes.len() - 1] >> 7) == 1);
        }
        bytes
    }

    fn pla(&mut self, bus: &Bus) {
        if self.registers.is_16bit_mode() {
            let bytes = self.do_pull(bus, 2);
            self.registers.a = (bytes[0] as u16) | ((bytes[1] as u16) << 8);
        } else {
            let bytes = self.do_pull(bus, 1);
            self.registers.set_low_a(bytes[0]);
        }
        self.increment_cycles_pla();
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
            // JMP
            0x4C => self.jmp(bus, A::Absolute),
            0x6C => self.jmp(bus, A::AbsoluteIndirect),
            0x7C => self.jmp(bus, A::AbsoluteIndexedIndirect(I::X)),
            0x5C => self.jmp(bus, A::AbsoluteLong),
            0xDC => self.jmp(bus, A::AbsoluteIndirectLong),
            // JSR
            0x20 => self.jsr(bus, A::Absolute),
            0xFC => self.jsr(bus, A::AbsoluteIndexedIndirect(I::X)),
            0x22 => self.jsr(bus, A::AbsoluteLong), // same as JSL
            // LDA
            0xA9 => self.lda(bus, A::Immediate),
            0xAD => self.lda(bus, A::Absolute),
            0xAF => self.lda(bus, A::AbsoluteLong),
            0xA5 => self.lda(bus, A::DirectPage),
            0xB2 => self.lda(bus, A::DirectPageIndirect),
            0xA7 => self.lda(bus, A::DirectPageIndirectLong),
            0xBD => self.lda(bus, A::AbsoluteIndexed(I::X)),
            0xBF => self.lda(bus, A::AbsoluteLongIndexed(I::X)),
            0xB9 => self.lda(bus, A::AbsoluteIndexed(I::Y)),
            0xB5 => self.lda(bus, A::DirectPageIndexed(I::X)),
            0xA1 => self.lda(bus, A::DirectPageIndexedIndirect(I::X)),
            0xB1 => self.lda(bus, A::DirectPageIndirectIndexed(I::Y)),
            0xB7 => self.lda(bus, A::DirectPageIndirectLongIndexed(I::Y)),
            0xA3 => self.lda(bus, A::StackRelative),
            0xB3 => self.lda(bus, A::StackRelativeIndirectIndexed(I::Y)),
            // LDX
            0xA2 => self.ldx(bus, A::Immediate),
            0xAE => self.ldx(bus, A::Absolute),
            0xA6 => self.ldx(bus, A::DirectPage),
            0xBE => self.ldx(bus, A::AbsoluteIndexed(I::Y)),
            0xB6 => self.ldx(bus, A::DirectPageIndexed(I::Y)),
            // LDY
            0xA0 => self.ldy(bus, A::Immediate),
            0xAC => self.ldy(bus, A::Absolute),
            0xA4 => self.ldy(bus, A::DirectPage),
            0xB4 => self.ldy(bus, A::AbsoluteIndexed(I::Y)),
            0xBC => self.ldy(bus, A::DirectPageIndexed(I::Y)),
            // LSR
            0x4A => self.lsr(bus, A::Accumulator),
            0x4E => self.lsr(bus, A::Absolute),
            0x46 => self.lsr(bus, A::DirectPage),
            0x5E => self.lsr(bus, A::AbsoluteIndexed(I::X)),
            0x56 => self.lsr(bus, A::DirectPageIndexed(I::X)),
            // MVN
            0x54 => unimplemented!("MVN instruction not implemented yet"),
            // MVP
            0x44 => unimplemented!("MVP instruction not implemented yet"),
            // NOP
            0xEA => self.nop(),
            // ORA
            0x09 => self.ora(bus, A::Immediate),
            0x0D => self.ora(bus, A::Absolute),
            0x0F => self.ora(bus, A::AbsoluteLong),
            0x05 => self.ora(bus, A::DirectPage),
            0x12 => self.ora(bus, A::DirectPageIndirect),
            0x07 => self.ora(bus, A::DirectPageIndirectLong),
            0x1D => self.ora(bus, A::AbsoluteIndexed(I::X)),
            0x1F => self.ora(bus, A::AbsoluteLongIndexed(I::X)),
            0x19 => self.ora(bus, A::AbsoluteIndexed(I::Y)),
            0x15 => self.ora(bus, A::DirectPageIndexed(I::X)),
            0x01 => self.ora(bus, A::DirectPageIndexedIndirect(I::X)),
            0x11 => self.ora(bus, A::DirectPageIndirectIndexed(I::Y)),
            0x17 => self.ora(bus, A::DirectPageIndirectLongIndexed(I::Y)),
            0x03 => self.ora(bus, A::StackRelative),
            0x13 => self.ora(bus, A::StackRelativeIndirectIndexed(I::Y)),
            // PEA
            0xF4 => self.pea(bus),
            // PEI
            0xD4 => self.pei(bus),
            // PER
            0x62 => self.per(bus),
            // PHA
            0x48 => self.pha(bus),
            // PHB
            0x8B => self.phb(bus),
            // PHD
            0x0B => self.phd(bus),
            // PHK
            0x4B => self.phk(bus),
            // PHP
            0x08 => self.php(bus),
            // PHX
            0xDA => self.phx(bus),
            // PHY
            0x5A => self.phy(bus),
            // PLA
            0x68 => self.pla(bus),
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
    fn test_ora() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.a   = 0x0F;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_memory_select_flag(true);
        bus.write(0x000001, 0xF0);
        cpu.ora(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.a, 0xFF);
        assert_eq!(cpu.registers.pc, 0x02);
        assert_eq!(cpu.cycles, 2);
        assert!(!cpu.registers.get_zero_flag());
        assert!(cpu.registers.get_negative_flag());
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
    fn test_lsr() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.a   = 0b00000000_00000011;
        cpu.registers.pbr = 0x00;
        cpu.registers.pc  = 0x0000;
        cpu.registers.set_memory_select_flag(false);
        cpu.registers.set_negative_flag(true);
        cpu.registers.set_carry_flag(false);
        cpu.lsr(&mut bus, AddressingMode::Accumulator);
        assert_eq!(cpu.registers.a, 0b00000000_00000001);
        assert_eq!(cpu.registers.pc, 0x01);
        assert_eq!(cpu.cycles, 4);
        assert!(cpu.registers.get_carry_flag());
        assert!(!cpu.registers.get_zero_flag());
        assert!(!cpu.registers.get_negative_flag());
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

    #[test]
    fn test_nop() {
        let mut cpu = CPU::new();
        cpu.registers.pc  = 0x0000;
        cpu.nop();
        assert_eq!(cpu.registers.pc, 0x01);
        assert_eq!(cpu.cycles, 2);
    }

    #[test]
    fn test_jmp() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        bus.write(0x000002, 0xAA);
        bus.write(0x000001, 0xBB);
        cpu.jmp(&bus, AddressingMode::Absolute);
        assert_eq!(cpu.registers.pc, 0xAABB);
        assert_eq!(cpu.cycles, 3);

        // Test a long address
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.registers.pbr  = 0x00;
        bus.write(0x000003, 0xAA);
        bus.write(0x000002, 0xBB);
        bus.write(0x000001, 0xCC);
        cpu.jmp(&bus, AddressingMode::AbsoluteLong);
        assert_eq!(cpu.registers.pbr, 0xAA);
        assert_eq!(cpu.registers.pc, 0xBBCC);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn test_jsr() {
        // Test a long address
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x1234;
        cpu.registers.pbr  = 0x00;
        cpu.registers.sp  = 0x1FC;
        bus.write(cpu.registers.get_pc_address() + 3, 0xAA);
        bus.write(cpu.registers.get_pc_address() + 2, 0xBB);
        bus.write(cpu.registers.get_pc_address() + 1, 0xCC);
        // write next instruction
        cpu.jsr(&mut bus, AddressingMode::AbsoluteLong);
        assert_eq!(bus.read(0x1FC), 0x00);
        assert_eq!(bus.read(0x1FB), 0x12);
        assert_eq!(bus.read(0x1FA), 0x38); // we should store the NEXT instruction
        assert_eq!(cpu.registers.pbr, 0xAA);
        assert_eq!(cpu.registers.pc, 0xBBCC);
        assert_eq!(cpu.registers.sp, 0x1F9);
        assert_eq!(cpu.cycles, 8);
    }

    #[test]
    fn test_lda() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.a  = 0x0000;
        cpu.registers.pc  = 0x0000;
        cpu.registers.pbr  = 0x00;
        cpu.registers.set_negative_flag(false);
        cpu.registers.set_zero_flag(true);
        cpu.registers.set_16bit_mode(false);
        bus.write(0x0001, 0xFF);
        cpu.lda(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.pc, 0x0002);
        assert_eq!(cpu.registers.a, 0x00FF);
        assert_eq!(cpu.cycles, 2);
        assert!(cpu.registers.get_negative_flag());
        assert!(!cpu.registers.get_zero_flag());
    }

    #[test]
    fn test_ldx() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.x  = 0x0000;
        cpu.registers.pc  = 0x0000;
        cpu.registers.pbr  = 0x00;
        cpu.registers.set_negative_flag(false);
        cpu.registers.set_zero_flag(true);
        cpu.registers.set_16bit_index(false);
        bus.write(0x0001, 0xFF);
        cpu.ldx(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.pc, 0x0002);
        assert_eq!(cpu.registers.x, 0x00FF);
        assert_eq!(cpu.cycles, 2);
        assert!(cpu.registers.get_negative_flag());
        assert!(!cpu.registers.get_zero_flag());
    }

    #[test]
    fn test_ldy() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.y  = 0x0000;
        cpu.registers.pc  = 0x0000;
        cpu.registers.pbr  = 0x00;
        cpu.registers.set_negative_flag(false);
        cpu.registers.set_zero_flag(true);
        cpu.registers.set_16bit_index(false);
        bus.write(0x0001, 0xFF);
        cpu.ldy(&bus, AddressingMode::Immediate);
        assert_eq!(cpu.registers.pc, 0x0002);
        assert_eq!(cpu.registers.y, 0x00FF);
        assert_eq!(cpu.cycles, 2);
        assert!(cpu.registers.get_negative_flag());
        assert!(!cpu.registers.get_zero_flag());
    }

    #[test]
    fn test_pea() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.registers.sp  = 0x1FC;
        bus.write(0x000002, 0xAA);
        bus.write(0x000001, 0xBB);
        cpu.pea(&mut bus);
        assert_eq!(bus.read(0x1FC), 0xAA);
        assert_eq!(bus.read(0x1FB), 0xBB);
        assert_eq!(cpu.registers.pc, 0x0003);
        assert_eq!(cpu.cycles, 5);
    }

    #[test]
    fn test_pei() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.registers.sp  = 0x1FC;
        cpu.registers.d = 0x00;
        bus.write(0x000001, 0x02); // Direct page address
        bus.write(0x000002, 0xAA);
        bus.write(0x000003, 0xBB);
        cpu.pei(&mut bus);
        assert_eq!(bus.read(0x1FC), 0xAA);
        assert_eq!(bus.read(0x1FB), 0xBB);
        assert_eq!(cpu.registers.pc, 0x0002);
        assert_eq!(cpu.cycles, 6);
    }

    #[test]
    fn test_per() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.registers.sp  = 0x1FC;
        bus.write(0x000002, 0x00);
        bus.write(0x000001, 0x01);
        cpu.per(&mut bus);
        assert_eq!(bus.read(0x1FC), 0x00);
        assert_eq!(bus.read(0x1FB), 0x04);
        assert_eq!(cpu.registers.pc, 0x0003);
        assert_eq!(cpu.cycles, 6);
    }

    #[test]
    fn test_pha() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.registers.sp  = 0x1FC;
        cpu.registers.a   = 0x1234;
        cpu.registers.set_16bit_mode(false);
        cpu.pha(&mut bus);
        assert_eq!(bus.read(0x1FC), 0x34);
        assert_eq!(cpu.registers.sp, 0x1FB);
        assert_eq!(cpu.registers.pc, 0x0001);
        assert_eq!(cpu.cycles, 3);
    }

    #[test]
    fn test_phb() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.registers.sp  = 0x1FC;
        cpu.registers.dbr   = 0x12;
        cpu.phb(&mut bus);
        assert_eq!(bus.read(0x1FC), 0x12);
        assert_eq!(cpu.registers.sp, 0x1FB);
        assert_eq!(cpu.registers.pc, 0x0001);
        assert_eq!(cpu.cycles, 3);
    }

    #[test]
    fn test_phd() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.registers.sp  = 0x1FC;
        cpu.registers.d = 0x1234;
        cpu.phd(&mut bus);
        assert_eq!(bus.read(0x1FC), 0x12);
        assert_eq!(bus.read(0x1FB), 0x34);
        assert_eq!(cpu.registers.sp, 0x1FA);
        assert_eq!(cpu.registers.pc, 0x0001);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn test_phk() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.registers.pbr  = 0x00;
        cpu.registers.sp  = 0x1FC;
        bus.write(0x1FC, 0xFF);
        cpu.phk(&mut bus);
        assert_eq!(bus.read(0x1FC), 0x00);
        assert_eq!(cpu.registers.sp, 0x1FB);
        assert_eq!(cpu.registers.pc, 0x0001);
        assert_eq!(cpu.cycles, 3);
    }

    #[test]
    fn test_php() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.registers.p  = 0x12;
        cpu.registers.sp  = 0x1FC;
        cpu.php(&mut bus);
        assert_eq!(bus.read(0x1FC), 0x12);
        assert_eq!(cpu.registers.sp, 0x1FB);
        assert_eq!(cpu.registers.pc, 0x0001);
        assert_eq!(cpu.cycles, 3);
    }

    #[test]
    fn test_phx() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.registers.x  = 0x1234;
        cpu.registers.sp  = 0x1FC;
        cpu.phx(&mut bus);
        assert_eq!(bus.read(0x1FC), 0x12);
        assert_eq!(bus.read(0x1FB), 0x34);
        assert_eq!(cpu.registers.sp, 0x1FA);
        assert_eq!(cpu.registers.pc, 0x0001);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn test_phy() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.registers.y  = 0x1234;
        cpu.registers.sp  = 0x1FC;
        cpu.phy(&mut bus);
        assert_eq!(bus.read(0x1FC), 0x12);
        assert_eq!(bus.read(0x1FB), 0x34);
        assert_eq!(cpu.registers.sp, 0x1FA);
        assert_eq!(cpu.registers.pc, 0x0001);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn test_pla() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.pc  = 0x0000;
        cpu.registers.y  = 0x1234;
        cpu.registers.set_negative_flag(true);
        cpu.registers.set_zero_flag(true);
        bus.write(0x1FB, 0x34);
        bus.write(0x1FC, 0x12);
        cpu.registers.sp  = 0x1FA;
        cpu.pla(&mut bus);
        assert_eq!(cpu.registers.a, 0x1234);
        assert_eq!(cpu.registers.sp, 0x1FC);
        assert_eq!(cpu.registers.pc, 0x0001);
        assert_eq!(cpu.registers.get_negative_flag(), false);
        assert_eq!(cpu.registers.get_zero_flag(), false);
        assert_eq!(cpu.cycles, 5);
    }
}
