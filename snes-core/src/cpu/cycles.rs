use crate::utils::addressing::{AddressingMode, IndexRegister};

use crate::cpu::registers::Registers;

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

const LDA_CONDITIONS: [Condition; 3] = [
    Condition::MemorySelectFlag,
    Condition::DirectPageIsZero,
    Condition::IndexCrossesPageBoundary,
];

const LD_INDEX_CONDITIONS: [Condition; 3] = [
    Condition::IndexIs16Bit,
    Condition::DirectPageIsZero,
    Condition::IndexCrossesPageBoundary,
];

const COMP_INDEX_CONDITIONS: [Condition; 2] = [
    Condition::IndexIs16Bit,
    Condition::IndexCrossesPageBoundary,
];

fn common_conditions(cpu_registers: &Registers, addressing_mode: AddressingMode, conditions: &[Condition]) -> (u16, usize) {
    let mut bytes = 0;
    let mut cycles = 0;

    for condition in conditions {
        match condition {
            // Add 1 byte and 1 cycle if m = 0 (16-bit memory/accumulator)
            Condition::MemorySelectFlag => {
                if cpu_registers.is_16bit_mode() {
                    cycles += 1;
                    if let A::Immediate = addressing_mode {
                        bytes += 1
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
                        if cpu_registers.direct_page_low() != 0 {
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
                        let page = cpu_registers.get_pc_address() & 0xFF;
                        let index = match index {
                            I::X => cpu_registers.x,
                            I::Y => cpu_registers.y,
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
                if cpu_registers.get_decimal_mode_flag() {
                    cycles += 1;
                }
            },
            // Add 1 byte if <index> = 0 (16-bit index registers)
            Condition::IndexIs16Bit => {
                if cpu_registers.is_16bit_index() {
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

pub fn increment_cycles_arithmetic(cpu_registers: &Registers, addressing_mode: AddressingMode) -> (u16, usize) {
    let mut total_bytes = 0;
    let mut total_cycles = 0;

    let (bytes, cycles) = common_bytes_cycles_arithmetic(addressing_mode);
    total_bytes += bytes; total_cycles += cycles;
    let (bytes, cycles) = common_conditions(cpu_registers, addressing_mode, &ALL_CONDITIONS);
    total_bytes += bytes; total_cycles += cycles;

    (total_bytes, total_cycles)
}

pub fn increment_cycles_bitwise(cpu_registers: &Registers, addressing_mode: AddressingMode) -> (u16, usize) {
    let mut total_bytes = 0;
    let mut total_cycles = 0;

    let (bytes, cycles) = common_bytes_cycles_arithmetic(addressing_mode);
    total_bytes += bytes; total_cycles += cycles;
    let (bytes, cycles) = common_conditions(cpu_registers, addressing_mode, &BITWISE_CONDITIONS);
    total_bytes += bytes; total_cycles += cycles;

    (total_bytes, total_cycles)
}

pub fn increment_cycles_shift(cpu_registers: &Registers, addressing_mode: AddressingMode) -> (u16, usize) {
    let mut total_bytes = 0;
    let mut total_cycles = 0;

    let (bytes, cycles) = common_bytes_cycles_shift(addressing_mode);
    total_bytes += bytes; total_cycles += cycles;
    // Add 2 cycles if m = 1
    let (_, cycles) = common_conditions(cpu_registers, addressing_mode, &[Condition::MemorySelectFlag]);
    total_cycles += cycles;
    let (bytes, cycles) = common_conditions(cpu_registers, addressing_mode, &BITWISE_CONDITIONS);
    total_bytes += bytes; total_cycles += cycles;

    (total_bytes, total_cycles)
}

pub fn increment_cycles_bit(cpu_registers: &Registers, addressing_mode: AddressingMode) -> (u16, usize) {
    let mut total_bytes = 0;
    let mut total_cycles = 0;

    let (bytes, cycles) = common_bytes_cycles_bit(addressing_mode);
    total_bytes += bytes; total_cycles += cycles;
    let (bytes, cycles) = common_conditions(cpu_registers, addressing_mode, &BITWISE_CONDITIONS);
    total_bytes += bytes; total_cycles += cycles;

    (total_bytes, total_cycles)
}

pub fn increment_cycles_clear() -> (u16, usize) {
    (1, 2)
}

pub fn increment_cycles_branch() -> (u16, usize) {
    (2, 2)
}

pub fn increment_cycles_branch_taken(page_boundary_crossed: bool) -> (u16, usize) {
    let mut total_cycles = 1;
    if page_boundary_crossed {
        total_cycles += 1;
    }

    (0, total_cycles)
}

pub fn increment_cycles_branch_long() -> (u16, usize) {
    (3, 4)
}

pub fn increment_cycles_comp_index(cpu_registers: &Registers, addressing_mode: AddressingMode) -> (u16, usize) {
    let mut total_bytes = 0;
    let mut total_cycles = 0;

    let (bytes, cycles) = common_bytes_cycles_arithmetic(addressing_mode);
    total_bytes += bytes; total_cycles += cycles;
    let (bytes, cycles) = common_conditions(cpu_registers, addressing_mode, &COMP_INDEX_CONDITIONS);
    total_bytes += bytes; total_cycles += cycles;

    (total_bytes, total_cycles)
}

pub fn increment_cycles_inc_dec(cpu_registers: &Registers, addressing_mode: AddressingMode) -> (u16, usize) {
    increment_cycles_shift(cpu_registers, addressing_mode)
}

pub fn increment_cycles_inc_dec_index() -> (u16, usize) {
    (1, 2)
}

pub fn increment_cycles_nop() -> (u16, usize) {
    (1, 2)
}

pub fn increment_cycles_wdm() -> (u16, usize) {
    (2, 2)
}

pub fn increment_cycles_jmp(addressing_mode: AddressingMode) -> (u16, usize) {
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
    (0, cycles)
}

/// Note: the bytes should be incremented *before* pushing onto the stack
pub fn increment_cycles_jsr(addressing_mode: AddressingMode) -> (u16, usize) {
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
    (bytes, cycles)
}

pub fn increment_cycles_lda(cpu_registers: &Registers, addressing_mode: AddressingMode) -> (u16, usize) {
    let mut total_bytes = 0;
    let mut total_cycles = 0;

    let (bytes, cycles) = common_bytes_cycles_arithmetic(addressing_mode);
    total_bytes += bytes; total_cycles += cycles;
    let (bytes, cycles) = common_conditions(cpu_registers, addressing_mode, &LDA_CONDITIONS);
    total_bytes += bytes; total_cycles += cycles;

    (total_bytes, total_cycles)
}

pub fn increment_cycles_ld_index(cpu_registers: &Registers, addressing_mode: AddressingMode) -> (u16, usize) {
    let mut total_bytes = 0;
    let mut total_cycles = 0;

    let (bytes, cycles) = common_bytes_cycles_arithmetic(addressing_mode);
    total_bytes += bytes; total_cycles += cycles;
    let (bytes, cycles) = common_conditions(cpu_registers, addressing_mode, &LD_INDEX_CONDITIONS);
    total_bytes += bytes; total_cycles += cycles;

    (total_bytes, total_cycles)
}

pub fn increment_cycles_pea() -> (u16, usize) {
    (3, 5)
}

pub fn increment_cycles_pei(cpu_registers: &Registers) -> (u16, usize) {
    let mut total_bytes = 2;
    let mut total_cycles = 6;

    let (bytes, cycles) = common_conditions(
        cpu_registers,
        AddressingMode::DirectPageIndirect,
        &[Condition::DirectPageIsZero],
    );
    total_bytes += bytes; total_cycles += cycles;

    (total_bytes, total_cycles)
}

pub fn increment_cycles_per() -> (u16, usize) {
    (3, 6)
}

pub fn increment_cycles_pha(is_16bit_mode: bool) -> (u16, usize) {
    (1, if is_16bit_mode {4} else {3})
}

pub fn increment_cycles_phb() -> (u16, usize) {
    (1, 3)
}

pub fn increment_cycles_phd() -> (u16, usize) {
    (1, 4)
}

pub fn increment_cycles_phk() -> (u16, usize) {
    (1, 3)
}

pub fn increment_cycles_php() -> (u16, usize) {
    (1, 3)
}

pub fn increment_cycles_push_index(is_16bit_index: bool) -> (u16, usize) {
    (1, if is_16bit_index {4} else {3})
}

pub fn increment_cycles_pla(is_16bit_mode: bool) -> (u16, usize) {
    (1, if is_16bit_mode {5} else {4})
}

pub fn increment_cycles_plb() -> (u16, usize) {
    (1, 4)
}

pub fn increment_cycles_pld() -> (u16, usize) {
    (1, 5)
}

pub fn increment_cycles_plp() -> (u16, usize) {
    (1, 4)
}

pub fn increment_cycles_pl_index(is_16bit_index: bool) -> (u16, usize) {
    (1, if is_16bit_index {5} else {4})
}

pub fn increment_cycles_rep() -> (u16, usize) {
    (2, 3)
}

pub fn increment_cycles_sep() -> (u16, usize) {
    (2, 3)
}

pub fn increment_cycles_return_subroutine() -> (u16, usize) {
    (0, 6)
}

pub fn increment_cycles_return_interrupt(is_emulation_mode: bool) -> (u16, usize) {
    (0, if is_emulation_mode {7} else {6})
}

pub fn increment_cycles_set_flag() -> (u16, usize) {
    (1, 2)
}

pub fn increment_cycles_sta(cpu_registers: &Registers, addressing_mode: AddressingMode) -> (u16, usize) {
    let mut total_bytes = 0;
    let mut total_cycles = 0;

    let (bytes, cycles) = common_bytes_cycles_arithmetic(addressing_mode);
    total_bytes += bytes; total_cycles += cycles;
    let (_, cycles) = common_conditions(cpu_registers, addressing_mode, &[
        Condition::MemorySelectFlag,
        Condition::DirectPageIsZero,
    ]);
    total_cycles += cycles;

    (total_bytes, total_cycles)
}

pub fn increment_cycles_st_index(cpu_registers: &Registers, addressing_mode: AddressingMode) -> (u16, usize) {
    let mut total_bytes = 0;
    let mut total_cycles = 0;

    let (bytes, cycles) = common_bytes_cycles_arithmetic(addressing_mode);
    total_bytes += bytes; total_cycles += cycles;
    let (_, cycles) = common_conditions(cpu_registers, addressing_mode, &[
        Condition::IndexIs16Bit,
        Condition::DirectPageIsZero,
    ]);
    total_cycles += cycles;

    (total_bytes, total_cycles)
}

pub fn increment_cycles_transfer() -> (u16, usize) {
    (1, 2)
}

pub fn increment_cycles_exchange() -> (u16, usize) {
    (1, 2)
}

pub fn increment_cycles_xba() -> (u16, usize) {
    (1, 3)
}

pub fn increment_cycles_brk(is_emulation_mode: bool) -> (u16, usize) {
    (2, if is_emulation_mode {8} else {7})
}

pub fn increment_cycles_stp() -> (u16, usize) {
    (1, 3)
}

pub fn increment_cycles_while_stopped() -> (u16, usize) {
    (0, 1)
}

pub fn increment_cycles_move(count: usize) -> (u16, usize) {
    (3, 7 * count)
}

pub fn increment_cycles_test(cpu_registers: &Registers, addressing_mode: AddressingMode) -> (u16, usize) {
    let mut total_bytes = 0;
    let mut total_cycles = 0;

    let (bytes, cycles) = match addressing_mode {
        AddressingMode::Absolute    => (3, 6),
        AddressingMode::DirectPage  => (2, 5),
        _ => unreachable!(),
    };
    total_bytes += bytes; total_cycles += cycles;
    let (_, cycles) = common_conditions(cpu_registers, addressing_mode, &[
        Condition::MemorySelectFlag,
        Condition::DirectPageIsZero,
    ]);
    total_cycles += cycles;

    (total_bytes, total_cycles)
}

#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test_common_conditions() {
        let mut registers = Registers::new();
        registers.emulation_mode = false;

        // 16-bit Memory/accumulator flag condition
        registers.pc = 0;
        registers.set_decimal_mode_flag(false);
        registers.set_16bit_mode(false);
        let (bytes, cycles) = common_conditions(&registers, AddressingMode::Immediate, &ALL_CONDITIONS);
        assert_eq!(bytes, 0);
        assert_eq!(cycles, 0);

        registers.pc = 0;
        registers.set_16bit_mode(true);
        let (bytes, cycles) = common_conditions(&registers, AddressingMode::Immediate, &ALL_CONDITIONS);
        assert_eq!(bytes, 1);
        assert_eq!(cycles, 1);

        // Decimal flag condition
        registers.pc = 0;
        registers.set_16bit_mode(true);
        registers.set_decimal_mode_flag(true);
        let (bytes, cycles) = common_conditions(&registers, AddressingMode::Immediate, &ALL_CONDITIONS);
        assert_eq!(bytes, 1);
        assert_eq!(cycles, 2);

        // Low byte of direct page register other than zero condition
        registers.pc = 0;
        registers.set_16bit_mode(false);
        registers.set_decimal_mode_flag(false);
        registers.d = 0x0000;
        let (bytes, cycles) = common_conditions(&registers, AddressingMode::DirectPage, &ALL_CONDITIONS);
        assert_eq!(bytes, 0);
        assert_eq!(cycles, 0);

        registers.pc = 0;
        registers.set_16bit_mode(false);
        registers.set_decimal_mode_flag(false);
        registers.d = 0x0001;
        let (bytes, cycles) = common_conditions(&registers, AddressingMode::DirectPage, &ALL_CONDITIONS);
        assert_eq!(bytes, 0);
        assert_eq!(cycles, 1);

        // Adding index crosses a page boundary condition
        registers.pc = 0xFE;
        registers.x = 0x0001;
        registers.set_16bit_mode(false);
        registers.set_decimal_mode_flag(false);
        let (bytes, cycles) = common_conditions(&registers, AddressingMode::AbsoluteIndexed(IndexRegister::X), &ALL_CONDITIONS);
        assert_eq!(bytes, 0);
        assert_eq!(cycles, 0); // Doesn't cross boundary

        registers.pc = 0xFE;
        registers.x = 0x0010;
        registers.set_16bit_mode(false);
        registers.set_decimal_mode_flag(false);
        let (bytes, cycles) = common_conditions(&registers, AddressingMode::AbsoluteIndexed(IndexRegister::X), &ALL_CONDITIONS);
        assert_eq!(bytes, 0);
        assert_eq!(cycles, 1); // Crosses boundary

        // Test common and aritmetic together
        registers.pc = 0xF5;
        registers.x = 0x0010;
        registers.set_16bit_mode(false);
        registers.set_decimal_mode_flag(false);
        let (bytes, cycles) = increment_cycles_arithmetic(&registers, AddressingMode::AbsoluteIndexed(IndexRegister::X));
        assert_eq!(bytes, 3);
        assert_eq!(cycles, 5);
    }
}
