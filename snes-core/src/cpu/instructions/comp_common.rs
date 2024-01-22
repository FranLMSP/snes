use crate::{cpu::registers::Registers, utils::{num_trait::SnesNum, alu}, common::flags::Flags};


pub fn do_comp<T: SnesNum>(registers: &mut Registers, target: T, value: T) {
    let (_, affected_flags) = alu::sbc_bin(target, value, true);
    for flag in affected_flags {
        match flag {
            Flags::Negative(_) |
            Flags::Zero(_) |
            Flags::Carry(_) => registers.set_flags(&[flag]),
            _ => {},
        }
    }
}