use crate::{cpu::registers::Registers, utils::{num_trait::SnesNum, alu}, common::flags::Flags};


pub fn do_comp<T: SnesNum>(registers: &mut Registers, target: T, value: T) {
    let (_, affected_flags) = alu::sbc_bin(target, value, false);
    for flag in affected_flags {
        match flag {
            Flags::Overflow(_) => {},
            _ => registers.set_flags(&[flag]),
        }
    }
}