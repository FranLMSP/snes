use crate::cpu::{registers::Registers, bus::Bus};


pub fn do_pull(registers: &mut Registers, bus: &mut Bus, count: usize, alter_flags: bool) -> Vec<u8> {
    let mut bytes = vec![];
    let mut is_zero = true;
    for _ in 0..count {
        registers.increment_sp(1);
        let mut address = registers.sp;
        if registers.emulation_mode {
            address = (registers.sp | 0x100) & 0x1FF
        }
        let byte = bus.read(address as u32);
        if byte != 0 {
            is_zero = false;
        }
        bytes.push(byte);
    }
    if alter_flags {
        registers.set_zero_flag(is_zero);
        if !bytes.is_empty() {
            // Low byte is pulled first, so we need to check
            // for the last byte that we pull
            registers.set_negative_flag((bytes[bytes.len() - 1] >> 7) == 1);
        }
    }
    bytes
}