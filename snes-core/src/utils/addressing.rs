use std::fmt::Display;

use crate::cpu::bus::Bus;

/// OPCODE #const
pub fn immediate(pc_addr: u32) -> u32 {
    pc_addr + 1
}

/// OPCODE addr
pub fn absolute(bus: &mut Bus, pc_addr: u32) -> u32 {
    (pc_addr & 0xFF0000) |
    (bus.read(pc_addr + 1) as u32) |
    ((bus.read(pc_addr + 2) as u32) << 8)
}

/// OPCODE (addr)
pub fn absolute_indirect(bus: &mut Bus, pc_addr: u32) -> u32 {
    let addr = absolute(bus, pc_addr);
    let dbr = pc_addr & 0xFF0000;
    dbr | ((bus.read(addr) as u32) << 8) | (bus.read(addr + 1) as u32)
}

/// OPCODE long
pub fn absolute_long(bus: &mut Bus, pc_addr: u32) -> u32 {
    (bus.read(pc_addr + 1) as u32) |
    ((bus.read(pc_addr + 2) as u32) << 8) |
    ((bus.read(pc_addr + 3) as u32) << 16)
}

/// OPCODE (addr)
pub fn absolute_indirect_long(bus: &mut Bus, pc_addr: u32) -> u32 {
    let addr = absolute(bus, pc_addr);
    ((bus.read(addr) as u32) << 16) |
    ((bus.read(addr + 1) as u32) << 8) |
    (bus.read(addr + 2) as u32)
}

/// OPCODE dp
pub fn direct_page(bus: &mut Bus, pc_addr: u32, direct_page_register: u16) -> u32 {
    (bus.read(pc_addr + 1) as u32) + direct_page_register as u32
}

/// OPCODE (dp)
pub fn direct_page_indirect(bus: &mut Bus, pc_addr: u32, direct_page_register: u16) -> u32 {
    let addr = direct_page(bus, pc_addr, direct_page_register);
    let dbr = pc_addr & 0xFF0000;
    dbr | ((bus.read(addr) as u32) << 8) | (bus.read(addr + 1) as u32)
}

/// OPCODE [dp]
pub fn direct_page_indirect_long(bus: &mut Bus, pc_addr: u32, direct_page_register: u16) -> u32 {
    let addr = direct_page(bus, pc_addr, direct_page_register);
    ((bus.read(addr) as u32) << 16) |
    ((bus.read(addr + 1) as u32) << 8) |
    (bus.read(addr + 2) as u32)
}

/// OPCODE addr,X
/// OPCODE addr,Y
pub fn absolute_indexed(bus: &mut Bus, pc_addr: u32, xy: u16) -> u32 {
    absolute(bus, pc_addr) + (xy as u32)
}

/// OPCODE (addr)
pub fn absolute_indexed_indirect(bus: &mut Bus, pc_addr: u32, xy: u16) -> u32 {
    let addr = absolute_indexed(bus, pc_addr, xy);
    let dbr = pc_addr & 0xFF0000;
    dbr | ((bus.read(addr) as u32) << 8) | (bus.read(addr + 1) as u32)
}


/// OPCODE long,X
/// OPCODE long,Y
pub fn absolute_long_indexed(bus: &mut Bus, pc_addr: u32, xy: u16) -> u32 {
    absolute_long(bus, pc_addr) + (xy as u32)
}

/// OPCODE dp,X
/// OPCODE dp,Y
pub fn direct_page_indexed(bus: &mut Bus, pc_addr: u32, direct_page_register: u16, xy: u16) -> u32 {
    direct_page(bus, pc_addr, direct_page_register) + (xy as u32)
}

/// OPCODE (dp,X)
/// OPCODE (dp,Y)
pub fn direct_page_indexed_indirect(bus: &mut Bus, pc_addr: u32, direct_page_register: u16, xy: u16) -> u32 {
    direct_page_indirect(bus, pc_addr, direct_page_register.wrapping_add(xy))
}

/// OPCODE (dp),X
/// OPCODE (dp),Y
pub fn direct_page_indirect_indexed(bus: &mut Bus, pc_addr: u32, direct_page_register: u16, xy: u16) -> u32 {
    direct_page_indirect(bus, pc_addr, direct_page_register) + (xy as u32)
}

/// OPCODE [dp],X
/// OPCODE [dp],Y
pub fn direct_page_indirect_long_indexed(bus: &mut Bus, pc_addr: u32, direct_page_register: u16, xy: u16) -> u32 {
    direct_page_indirect_long(bus, pc_addr, direct_page_register) + (xy as u32)
}

/// OPCODE sr,S
pub fn stack_relative(bus: &mut Bus, pc_addr: u32, stack_pointer: u16) -> u32 {
    absolute_indexed(bus, pc_addr, stack_pointer)
}

/// OPCODE (sr,S),X
/// OPCODE (sr,S),Y
pub fn stack_relative_indirect_indexed(bus: &mut Bus, pc_addr: u32, stack_pointer: u16, xy: u16) -> u32 {
    absolute_indexed(bus, pc_addr, stack_pointer) + (xy as u32)
}

#[derive(Copy, Clone, PartialEq)]
pub enum IndexRegister {
    X, Y,
}

impl Display for IndexRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            IndexRegister::X => "X",
            IndexRegister::Y => "Y",
        };
        write!(f, "{}", name)
    }
}

#[derive(Copy, Clone)]
pub enum AddressingMode {
    Accumulator,
    Immediate,
    Absolute,
    AbsoluteIndirect,
    AbsoluteIndirectLong,
    AbsoluteLong,
    DirectPage,
    DirectPageIndirect,
    DirectPageIndirectLong,
    AbsoluteIndexed(IndexRegister),
    AbsoluteIndexedIndirect(IndexRegister),
    AbsoluteLongIndexed(IndexRegister),
    DirectPageIndexed(IndexRegister),
    DirectPageIndexedIndirect(IndexRegister),
    DirectPageIndirectIndexed(IndexRegister),
    DirectPageIndirectLongIndexed(IndexRegister),
    StackRelative,
    StackRelativeIndirectIndexed(IndexRegister),
}

impl AddressingMode {
    pub fn effective_address(self, bus: &mut Bus, pc_addr: u32, direct_page_register: u16, stack_pointer: u16, x: u16, y: u16) -> u32 {
        use IndexRegister::X as X;
        // TODO: maybe use impl Immediate {pub fn effective_address} to prevent this match statement?
        match self {
            Self::Accumulator => pc_addr,
            Self::Immediate => immediate(pc_addr),
            Self::Absolute => absolute(bus, pc_addr),
            Self::AbsoluteIndirect => absolute_indirect(bus, pc_addr),
            Self::AbsoluteIndirectLong => absolute_indirect_long(bus, pc_addr),
            Self::AbsoluteLong => absolute_long(bus, pc_addr),
            Self::DirectPage => direct_page(bus, pc_addr, direct_page_register),
            Self::DirectPageIndirect => direct_page_indirect(bus, pc_addr, direct_page_register),
            Self::DirectPageIndirectLong => direct_page_indirect_long(bus, pc_addr, direct_page_register),
            Self::AbsoluteIndexed(idx) => absolute_indexed(bus, pc_addr, if idx == X {x} else {y}),
            Self::AbsoluteIndexedIndirect(idx) => absolute_indexed_indirect(bus, pc_addr, if idx == X {x} else {y}),
            Self::AbsoluteLongIndexed(idx) => absolute_long_indexed(bus, pc_addr, if idx == X {x} else {y}),
            Self::DirectPageIndexed(idx) => direct_page_indexed(bus, pc_addr, direct_page_register, if idx == X {x} else {y}),
            Self::DirectPageIndexedIndirect(idx) => direct_page_indexed_indirect(bus, pc_addr, direct_page_register, if idx == X {x} else {y}),
            Self::DirectPageIndirectIndexed(idx) => direct_page_indirect_indexed(bus, pc_addr, direct_page_register, if idx == X {x} else {y}),
            Self::DirectPageIndirectLongIndexed(idx) => direct_page_indirect_long_indexed(bus, pc_addr, direct_page_register, if idx == X {x} else {y}),
            Self::StackRelative => stack_relative(bus, pc_addr, stack_pointer),
            Self::StackRelativeIndirectIndexed(idx) => stack_relative_indirect_indexed(bus, pc_addr, stack_pointer, if idx == X {x} else {y}),
        }
    }

    pub fn value_8bit(self, bus: &mut Bus, pc_addr: u32, direct_page_register: u16, stack_pointer: u16, x: u16, y: u16) -> u8 {
        let address = self.effective_address(bus, pc_addr, direct_page_register, stack_pointer, x, y);
        return bus.read(address);
    }

    pub fn value_16bit(self, bus: &mut Bus, pc_addr: u32, direct_page_register: u16, stack_pointer: u16, x: u16, y: u16) -> u16 {
        let address = self.effective_address(bus, pc_addr, direct_page_register, stack_pointer, x, y);
        return (bus.read(address) as u16) | ((bus.read(address + 1) as u16) << 8);
    }

    pub fn store_8bit(self, bus: &mut Bus, pc_addr: u32, direct_page_register: u16, stack_pointer: u16, x: u16, y: u16, value: u8) {
        let address = self.effective_address(bus, pc_addr, direct_page_register, stack_pointer, x, y);
        bus.write(address, value);
    }

    pub fn store_16bit(self, bus: &mut Bus, pc_addr: u32, direct_page_register: u16, stack_pointer: u16, x: u16, y: u16, value: u16) {
        let address = self.effective_address(bus, pc_addr, direct_page_register, stack_pointer, x, y);
        bus.write(address, value as u8);
        bus.write(address + 1, (value >> 8) as u8);
    }
}

#[cfg(test)]
mod addressing_modes_tests {
    use super::*;

    #[test]
    fn test_immediate() {
        let pc_addr = 0x000000;
        assert_eq!(immediate(pc_addr),  0x0001);
    }

    #[test]
    fn test_absolute() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(absolute(&mut bus, pc_addr), 0x000201);

        let mut bus = Bus::new();
        let pc_addr = 0x7F0000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(absolute(&mut bus, pc_addr), 0x7F0201);
    }

    #[test]
    fn test_absolute_indirect() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let addr = 0x55;
        bus.write(pc_addr + 1, addr);
        bus.write(addr as u32, 0x02);
        bus.write((addr + 1) as u32, 0x01);
        assert_eq!(absolute_indirect(&mut bus, pc_addr), 0x000201);

        let pc_addr = 0x7E0010;
        let addr = 0x55;
        bus.write(pc_addr + 1, addr);
        bus.write(addr as u32, 0x02);
        bus.write((addr + 1) as u32, 0x01);
        assert_eq!(absolute_indirect(&mut bus, pc_addr), 0x7E0201);

        let pc_addr = 0x7E0010;
        let addr = 0x55;
        bus.write(pc_addr + 1, addr);
        bus.write(addr as u32, 0x02);
        bus.write((addr + 1) as u32, 0x01);
        assert_eq!(absolute_indirect(&mut bus, pc_addr), 0x7E0201);
    }

    #[test]
    fn test_absolute_long() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        bus.write(pc_addr + 3, 0x03);
        assert_eq!(absolute_long(&mut bus, pc_addr), 0x030201);

        let mut bus = Bus::new();
        let pc_addr = 0x7E0010;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        bus.write(pc_addr + 3, 0x03);
        assert_eq!(absolute_long(&mut bus, pc_addr), 0x030201);
    }

    #[test]
    fn test_absolute_indirect_long() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let addr = 0x55;
        bus.write(pc_addr + 1, addr);
        bus.write(addr as u32, 0x03);
        bus.write((addr + 1) as u32, 0x02);
        bus.write((addr + 2) as u32, 0x01);
        assert_eq!(absolute_indirect_long(&mut bus, pc_addr), 0x030201);
    }


    #[test]
    fn test_direct_page() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        assert_eq!(direct_page(&mut bus, pc_addr, 0x00), 0x000055);

        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        assert_eq!(direct_page(&mut bus, pc_addr, 0x01), 0x000056);
    }

    #[test]
    fn test_direct_page_indirect() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write(dp as u32, 0x02);
        bus.write((dp + 1) as u32, 0x01);
        assert_eq!(direct_page_indirect(&mut bus, pc_addr, 0x00), 0x000201);

        let pc_addr = 0x7E0010;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write(dp as u32, 0x02);
        bus.write((dp + 1) as u32, 0x01);
        assert_eq!(direct_page_indirect(&mut bus, pc_addr, 0x00), 0x7E0201);

        let pc_addr = 0x7E0010;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write((dp + 1) as u32, 0x02);
        bus.write((dp + 2) as u32, 0x01);
        assert_eq!(direct_page_indirect(&mut bus, pc_addr, 0x01), 0x7E0201);
    }

    #[test]
    fn test_direct_page_indirect_long() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write(dp as u32, 0x03);
        bus.write((dp + 1) as u32, 0x02);
        bus.write((dp + 2) as u32, 0x01);
        assert_eq!(direct_page_indirect_long(&mut bus, pc_addr, 0x00), 0x030201);

        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write((dp + 1) as u32, 0x03);
        bus.write((dp + 2) as u32, 0x02);
        bus.write((dp + 3) as u32, 0x01);
        assert_eq!(direct_page_indirect_long(&mut bus, pc_addr, 0x01), 0x030201);
    }

    #[test]
    fn test_absolute_indexed() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(absolute_indexed(&mut bus, pc_addr, 0x02), 0x000203);

        let mut bus = Bus::new();
        let pc_addr = 0x7F0000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(absolute_indexed(&mut bus, pc_addr, 0x02), 0x7F0203);
    }

    #[test]
    fn test_absolute_indexed_indirect() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let addr = 0x55;
        bus.write(pc_addr + 1, addr);
        bus.write(addr as u32, 0x02);
        bus.write((addr + 1) as u32, 0x01);
        assert_eq!(absolute_indexed_indirect(&mut bus, pc_addr, 0), 0x000201);
    }

    #[test]
    fn test_absolute_long_indexed() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        bus.write(pc_addr + 3, 0x03);
        assert_eq!(absolute_long_indexed(&mut bus, pc_addr, 0x02), 0x030203);

        let mut bus = Bus::new();
        let pc_addr = 0x7E0010;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        bus.write(pc_addr + 3, 0x03);
        assert_eq!(absolute_long_indexed(&mut bus, pc_addr, 0x02), 0x030203);
    }

    #[test]
    fn test_direct_page_indexed() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        assert_eq!(direct_page_indexed(&mut bus, pc_addr, 0x00, 0x01), 0x000056);

        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        assert_eq!(direct_page_indexed(&mut bus, pc_addr, 0x01, 0x01), 0x000057);
    }

    #[test]
    fn test_direct_page_indexed_indirect() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write((dp + 1) as u32, 0x02);
        bus.write((dp + 2) as u32, 0x01);
        assert_eq!(direct_page_indexed_indirect(&mut bus, pc_addr, 0x00, 0x01), 0x000201);

        let pc_addr = 0x7E0010;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write((dp + 1) as u32, 0x02);
        bus.write((dp + 2) as u32, 0x01);
        assert_eq!(direct_page_indexed_indirect(&mut bus, pc_addr, 0x00, 0x01), 0x7E0201);

        let pc_addr = 0x7E0010;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write((dp + 2) as u32, 0x02);
        bus.write((dp + 3) as u32, 0x01);
        assert_eq!(direct_page_indexed_indirect(&mut bus, pc_addr, 0x01, 0x01), 0x7E0201);
    }

    #[test]
    fn test_direct_page_indirect_indexed() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write(dp as u32, 0x02);
        bus.write((dp + 1) as u32, 0x01);
        assert_eq!(direct_page_indirect_indexed(&mut bus, pc_addr, 0x00, 0x01), 0x000202);

        let pc_addr = 0x7E0010;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write(dp as u32, 0x02);
        bus.write((dp + 1) as u32, 0x01);
        assert_eq!(direct_page_indirect_indexed(&mut bus, pc_addr, 0x00, 0x01), 0x7E0202);

        let pc_addr = 0x7E0010;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write((dp + 1) as u32, 0x02);
        bus.write((dp + 2) as u32, 0x01);
        assert_eq!(direct_page_indirect_indexed(&mut bus, pc_addr, 0x01, 0x01), 0x7E0202);
    }

    #[test]
    fn test_direct_page_indirect_long_indexed() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write(dp as u32, 0x03);
        bus.write((dp + 1) as u32, 0x02);
        bus.write((dp + 2) as u32, 0x01);
        assert_eq!(direct_page_indirect_long_indexed(&mut bus, pc_addr, 0x00, 0x02), 0x030203);

        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write((dp + 1) as u32, 0x03);
        bus.write((dp + 2) as u32, 0x02);
        bus.write((dp + 3) as u32, 0x01);
        assert_eq!(direct_page_indirect_long_indexed(&mut bus, pc_addr, 0x01, 0x02), 0x030203);
    }

    #[test]
    fn test_stack_relative() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(stack_relative(&mut bus, pc_addr, 0x02), 0x000203);

        let mut bus = Bus::new();
        let pc_addr = 0x7F0000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(stack_relative(&mut bus, pc_addr, 0x02), 0x7F0203);
    }

    #[test]
    fn test_stack_relative_indirect_indexed() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(stack_relative_indirect_indexed(&mut bus, pc_addr, 0x02, 0x02), 0x000205);

        let mut bus = Bus::new();
        let pc_addr = 0x7F0000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(stack_relative_indirect_indexed(&mut bus, pc_addr, 0x02, 0x02), 0x7F0205);
    }

    #[test]
    fn test_address_value() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x20);
        bus.write(pc_addr + 2, 0x10);
        bus.write(0x001020, 0xFE);
        let val = AddressingMode::Absolute.value_8bit(&mut bus, pc_addr, 0x00, 0x00, 0x00, 0x00);
        assert_eq!(val, 0xFE);

        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x20);
        bus.write(pc_addr + 2, 0x10);
        bus.write(0x001020, 0xFF);
        bus.write(0x001021, 0xEE);
        let val = AddressingMode::Absolute.value_16bit(&mut bus, pc_addr, 0x00, 0x00, 0x00, 0x00);
        assert_eq!(val, 0xEEFF);
    }
}