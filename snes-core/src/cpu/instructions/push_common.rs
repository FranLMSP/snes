use crate::cpu::{registers::Registers, bus::Bus};

pub fn do_push(registers: &mut Registers, bus: &mut Bus, bytes: &[u8]) {
    for byte in bytes {
        let mut address = registers.sp as u32;
        if registers.emulation_mode {
            address = (address & 0xFF) | 0x100;
        }
        bus.write(address, *byte);
        registers.decrement_sp(1);
    }
}
