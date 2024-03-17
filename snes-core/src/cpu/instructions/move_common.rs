use crate::cpu::{registers::Registers, bus::Bus, cycles};

pub fn do_move(registers: &mut Registers, bus: &mut Bus, is_next: bool) {
    let pc = registers.get_pc_address();
    let source_bank = bus.read(pc + 2);
    let dest_bank = bus.read(pc + 1);
    let mut count = 0;
    while registers.a != 0xFFFF {
        let (x, y) = match registers.is_16bit_index() {
            true => (registers.x, registers.y),
            false => (registers.x & 0x00FF, registers.y & 0x00FF),
        };
        let source_address = ((source_bank as u32) << 16) | (x as u32);
        let dest_address = ((dest_bank as u32) << 16) | (y as u32);
        let byte = bus.read(source_address);
        bus.write(dest_address, byte);
        registers.a = registers.a.wrapping_sub(1);
        if is_next {
            registers.x = registers.x.wrapping_add(1);
            registers.y = registers.y.wrapping_add(1);
        } else {
            registers.x = registers.x.wrapping_sub(1);
            registers.y = registers.y.wrapping_sub(1);
        }
        count += 1;
    }
    let (bytes, cycles) = cycles::increment_cycles_move(count);
    registers.increment_pc(bytes); registers.cycles += cycles;
}


pub fn tick_move(registers: &mut Registers, bus: &mut Bus, is_next: bool) {
    let pc = registers.get_pc_address();
    // We assume that the 3 bytes of the instructions were already fetched
    let source_bank = bus.read(pc.wrapping_sub(1));
    let dest_bank = bus.read(pc.wrapping_sub(2));
    let source_address = ((source_bank as u32) << 16) | (registers.x as u32);
    let dest_address = ((dest_bank as u32) << 16) | (registers.y as u32);
    let byte = bus.read(source_address);
    bus.write(dest_address, byte);
    if is_next {
        registers.x = registers.x.wrapping_add(1);
        registers.y = registers.y.wrapping_add(1);
    } else {
        registers.x = registers.x.wrapping_sub(1);
        registers.y = registers.y.wrapping_sub(1);
    }
    registers.a = registers.a.wrapping_sub(1);
    if registers.a == 0xFFFF {
        registers.dbr = dest_bank;
        registers.is_moving = false;
    }
    registers.cycles += 7;
}
