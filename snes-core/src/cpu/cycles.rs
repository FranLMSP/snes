use super::cpu::CPU;
use crate::utils::addressing::{AddressingMode, IndexRegister};

type A = AddressingMode;
type I = IndexRegister;

enum Condition {
    MemorySelectFlag,
    DirectPageIsZero,
    IndexCrossesPageBoundary,
    DecimalMode,
    IndexIs16Bit,
}

const ALL_CONDITIONS: [Condition; 4] = [
    Condition::MemorySelectFlag,
    Condition::DirectPageIsZero,
    Condition::IndexCrossesPageBoundary,
    Condition::DecimalMode,
];

const BITWISE_CONDITIONS: [Condition; 3] = [
    Condition::MemorySelectFlag,
    Condition::DirectPageIsZero,
    Condition::IndexCrossesPageBoundary,
];

const COMP_INDEX_CONDITIONS: [Condition; 2] = [
    Condition::IndexIs16Bit,
    Condition::IndexCrossesPageBoundary,
];

impl CPU {
    fn common_conditions(&mut self, addressing_mode: AddressingMode, conditions: &[Condition]) -> (u16, usize) {
        let mut bytes = 0;
        let mut cycles = 0;

        for condition in conditions {
            match condition {
                // Add 1 byte and 1 cycle if m = 0 (16-bit memory/accumulator)
                Condition::MemorySelectFlag => {
                    if !self.registers.get_memory_select_flag() {
                        cycles += 1;
                        match addressing_mode {
                            A::Immediate => bytes += 1,
                            _ => {},
                        }
                    }
                },
                // Add 1 cycle if low byte of Direct Page register is other than zero (DL< >0)
                Condition::DirectPageIsZero => {
                    match addressing_mode {
                        A::DirectPage | A::DirectPageIndirect | A::DirectPageIndirectLong |
                        A::DirectPageIndexed(_) | A::DirectPageIndexedIndirect(_) |
                        A::DirectPageIndirectIndexed(_) |
                        A::DirectPageIndirectLongIndexed(_) => {
                            if self.registers.direct_page_low() != 0 {
                                cycles += 1;
                            }
                        },
                        _ => {},
                    };
                },
                // Add 1 cycle if adding index crosses a page boundary
                Condition::IndexCrossesPageBoundary => {
                    match addressing_mode {
                        A::AbsoluteIndexed(index) | A::DirectPageIndirectIndexed(index) => {
                            let page = self.registers.get_pc_address() & 0xFF;
                            let index = match index {
                                I::X => self.registers.x,
                                I::Y => self.registers.y,
                            };
                            if (page + index as u32) > 0xFF {
                                cycles += 1
                            }
                        },
                        _ => {},
                    };
                },
                // Add 1 cycle if 65C02 and d = 1 (decimal mode, 65C02)
                Condition::DecimalMode => {
                    if self.registers.get_decimal_mode_flag() {
                        cycles += 1;
                    }
                },
                // Add 1 byte if <index> = 0 (16-bit index registers)
                Condition::IndexIs16Bit => {
                    if self.registers.is_16bit_index() {
                        bytes += 1; cycles += 1;
                    }
                },
            };
        }

        (bytes, cycles)
    }

    fn common_bytes_cycles_arithmetic(addressing_mode: AddressingMode) -> (u16, usize) {
        match addressing_mode {
            A::Immediate                        => (2, 2),
            A::Absolute                         => (3, 4),
            A::AbsoluteLong                     => (4, 5),
            A::DirectPage                       => (2, 3),
            A::DirectPageIndirect               => (2, 5),
            A::DirectPageIndirectLong           => (2, 6),
            A::AbsoluteIndexed(_)               => (3, 4),
            A::AbsoluteLongIndexed(_)           => (4, 5),
            A::DirectPageIndexed(_)             => (2, 4),
            A::DirectPageIndexedIndirect(_)     => (2, 6),
            A::DirectPageIndirectIndexed(_)     => (2, 5),
            A::DirectPageIndirectLongIndexed(_) => (2, 6),
            A::StackRelative                    => (2, 4),
            A::StackRelativeIndirectIndexed(_)  => (2, 7),
            _ => unreachable!(),
        }
    }

    fn common_bytes_cycles_shift(addressing_mode: AddressingMode) -> (u16, usize) {
        match addressing_mode {
            A::Accumulator                      => (1, 2),
            A::Absolute                         => (3, 6),
            A::DirectPage                       => (2, 5),
            // Note: in some documentations you will find that this addressing mode has
            // 7 cycles for shift instructions, and then it says to substract
            // 1 cycles if no page boundary is crossed.
            // But to make it simpler, we are assigning 6 cycles here and then incrementing
            // it by 1 later if a page boundary is crossed.
            A::AbsoluteIndexed(_)               => (3, 6),
            A::DirectPageIndexed(_)             => (2, 6),
            _ => unreachable!(),
        }
    }

    fn common_bytes_cycles_bit(addressing_mode: AddressingMode) -> (u16, usize) {
        match addressing_mode {
            A::Immediate                        => (2, 2),
            A::Absolute                         => (3, 4),
            A::DirectPage                       => (2, 3),
            A::AbsoluteIndexed(_)               => (3, 4),
            A::DirectPageIndexed(_)             => (2, 4),
            _ => unreachable!(),
        }
    }

    pub fn increment_cycles_arithmetic(&mut self, addressing_mode: AddressingMode) {
        let (bytes, cycles) = CPU::common_bytes_cycles_arithmetic(addressing_mode);
        self.registers.increment_pc(bytes); self.cycles += cycles;
        let (bytes, cycles) = self.common_conditions(addressing_mode, &ALL_CONDITIONS);
        self.registers.increment_pc(bytes); self.cycles += cycles;
    }

    pub fn increment_cycles_bitwise(&mut self, addressing_mode: AddressingMode) {
        let (bytes, cycles) = CPU::common_bytes_cycles_arithmetic(addressing_mode);
        self.registers.increment_pc(bytes); self.cycles += cycles;
        let (bytes, cycles) = self.common_conditions(addressing_mode, &BITWISE_CONDITIONS);
        self.registers.increment_pc(bytes); self.cycles += cycles;
    }

    pub fn increment_cycles_shift(&mut self, addressing_mode: AddressingMode) {
        let (bytes, cycles) = CPU::common_bytes_cycles_shift(addressing_mode);
        self.registers.increment_pc(bytes); self.cycles += cycles;
        // Add 2 cycles if m = 1
        let (_, cycles) = self.common_conditions(addressing_mode, &[Condition::MemorySelectFlag]);
        self.cycles += cycles;
        let (bytes, cycles) = self.common_conditions(addressing_mode, &BITWISE_CONDITIONS);
        self.registers.increment_pc(bytes); self.cycles += cycles;
    }

    pub fn increment_cycles_bit(&mut self, addressing_mode: AddressingMode) {
        let (bytes, cycles) = CPU::common_bytes_cycles_bit(addressing_mode);
        self.registers.increment_pc(bytes); self.cycles += cycles;
        let (bytes, cycles) = self.common_conditions(addressing_mode, &BITWISE_CONDITIONS);
        self.registers.increment_pc(bytes); self.cycles += cycles;
    }

    pub fn increment_cycles_clear(&mut self) {
        self.registers.increment_pc(1); self.cycles += 2;
    }

    pub fn increment_cycles_branch(&mut self) {
        self.registers.increment_pc(2); self.cycles += 2;
    }

    pub fn increment_cycles_branch_taken(&mut self, page_boundary_crossed: bool) {
        self.cycles += 1;
        if page_boundary_crossed {
            self.cycles += 1;
        }
    }

    pub fn increment_cycles_branch_long(&mut self) {
        self.registers.increment_pc(3); self.cycles += 4;
    }

    pub fn increment_cycles_comp_index(&mut self, addressing_mode: AddressingMode) {
        let (bytes, cycles) = CPU::common_bytes_cycles_arithmetic(addressing_mode);
        self.registers.increment_pc(bytes); self.cycles += cycles;
        let (bytes, cycles) = self.common_conditions(addressing_mode, &COMP_INDEX_CONDITIONS);
        self.registers.increment_pc(bytes); self.cycles += cycles;
    }

    pub fn increment_cycles_inc_dec(&mut self, addressing_mode: AddressingMode) {
        self.increment_cycles_shift(addressing_mode);
    }

    pub fn increment_cycles_inc_dec_index(&mut self) {
        self.registers.increment_pc(1); self.cycles += 2;
    }

    pub fn increment_cycles_nop(&mut self) {
        self.registers.increment_pc(1); self.cycles += 2;
    }

    pub fn increment_cycles_jmp(&mut self, addressing_mode: AddressingMode) {
        let (_, cycles) = match addressing_mode {
            A::Absolute                         => (3, 3),
            A::AbsoluteIndirect                 => (3, 5),
            A::AbsoluteIndexedIndirect(_)       => (3, 6),
            A::AbsoluteLong                     => (4, 4),
            A::AbsoluteIndirectLong             => (3, 6),
            _ => unreachable!(),
        };
        // Incrementing PC here is kind of irrelevant since we
        // are performing a JMP anyway.
        // However, we have to keep in mind PBR if we happen to increment PC at 0xFFFF
        // self.registers.increment_pc(bytes); // TODO: consider above comment
        self.cycles += cycles;
    }

    /// Note: the bytes should be incremented *before* pushing onto the stack
    pub fn increment_cycles_jsr(&mut self, addressing_mode: AddressingMode) {
        let (bytes, cycles) = match addressing_mode {
            A::Absolute                         => (3, 6),
            A::AbsoluteIndexedIndirect(_)       => (3, 8),
            A::AbsoluteLong                     => (4, 8),
            _ => unreachable!(),
        };
        // Incrementing PC here is kind of irrelevant since we
        // are performing a JMP anyway.
        // However, we have to keep in mind PBR if we happen to increment PC at 0xFFFF
        // self.registers.increment_pc(bytes); // TODO: consider above comment
        self.registers.increment_pc(bytes); self.cycles += cycles;
    }
}

#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test_common_conditions() {
        let mut cpu = CPU::new();

        // 16-bit Memory/accumulator flag condition
        cpu.registers.pc = 0;
        cpu.registers.set_decimal_mode_flag(false);
        cpu.registers.set_16bit_mode(false);
        let (bytes, cycles) = cpu.common_conditions(AddressingMode::Immediate, &ALL_CONDITIONS);
        assert_eq!(bytes, 0);
        assert_eq!(cycles, 0);

        cpu.registers.pc = 0;
        cpu.registers.set_16bit_mode(true);
        let (bytes, cycles) = cpu.common_conditions(AddressingMode::Immediate, &ALL_CONDITIONS);
        assert_eq!(bytes, 1);
        assert_eq!(cycles, 1);

        // Decimal flag condition
        cpu.registers.pc = 0;
        cpu.registers.set_16bit_mode(true);
        cpu.registers.set_decimal_mode_flag(true);
        let (bytes, cycles) = cpu.common_conditions(AddressingMode::Immediate, &ALL_CONDITIONS);
        assert_eq!(bytes, 1);
        assert_eq!(cycles, 2);

        // Low byte of direct page register other than zero condition
        cpu.registers.pc = 0;
        cpu.registers.set_16bit_mode(false);
        cpu.registers.set_decimal_mode_flag(false);
        cpu.registers.d = 0x0000;
        let (bytes, cycles) = cpu.common_conditions(AddressingMode::DirectPage, &ALL_CONDITIONS);
        assert_eq!(bytes, 0);
        assert_eq!(cycles, 0);

        cpu.registers.pc = 0;
        cpu.registers.set_16bit_mode(false);
        cpu.registers.set_decimal_mode_flag(false);
        cpu.registers.d = 0x0001;
        let (bytes, cycles) = cpu.common_conditions(AddressingMode::DirectPage, &ALL_CONDITIONS);
        assert_eq!(bytes, 0);
        assert_eq!(cycles, 1);

        // Adding index crosses a page boundary condition
        cpu.registers.pc = 0xFE;
        cpu.registers.x = 0x0001;
        cpu.registers.set_16bit_mode(false);
        cpu.registers.set_decimal_mode_flag(false);
        let (bytes, cycles) = cpu.common_conditions(AddressingMode::AbsoluteIndexed(IndexRegister::X), &ALL_CONDITIONS);
        assert_eq!(bytes, 0);
        assert_eq!(cycles, 0); // Doesn't cross boundary

        cpu.registers.pc = 0xFE;
        cpu.registers.x = 0x0010;
        cpu.registers.set_16bit_mode(false);
        cpu.registers.set_decimal_mode_flag(false);
        let (bytes, cycles) = cpu.common_conditions(AddressingMode::AbsoluteIndexed(IndexRegister::X), &ALL_CONDITIONS);
        assert_eq!(bytes, 0);
        assert_eq!(cycles, 1); // Crosses boundary

        // Test common and aritmetic together
        cpu.registers.pc = 0xF5;
        cpu.registers.x = 0x0010;
        cpu.cycles = 0;
        cpu.registers.set_16bit_mode(false);
        cpu.registers.set_decimal_mode_flag(false);
        cpu.increment_cycles_arithmetic(AddressingMode::AbsoluteIndexed(IndexRegister::X));
        assert_eq!(cpu.registers.pc, 0xF5 + 3);
        assert_eq!(cpu.cycles, 5);
    }
}