use std::fmt::Display;

use crate::cpu::bus::Bus;


#[derive(Copy, Clone)]
pub struct ResolveAddressParams {
    pub pc_addr: u32,
    pub dbr: u8,
    pub direct_page_register: u16,
    pub stack_pointer: u16,
    pub x: u16,
    pub y: u16,
}


/// OPCODE #const
pub fn immediate(pc_addr: u32) -> u32 {
    pc_addr + 1
}

/// OPCODE addr
pub fn absolute(bus: &mut Bus, pc_addr: u32, dbr: u8) -> u32 {
    let addr = (bus.read(pc_addr + 1) as u32) | ((bus.read(pc_addr + 2) as u32) << 8);
    ((dbr as u32) << 16) | addr
}

/// OPCODE (addr)
pub fn absolute_indirect(bus: &mut Bus, pc_addr: u32, dbr: u8) -> u32 {
    let addr = absolute(bus, pc_addr, dbr);
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
pub fn absolute_indirect_long(bus: &mut Bus, pc_addr: u32, dbr: u8) -> u32 {
    let addr = absolute(bus, pc_addr, dbr);
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
pub fn absolute_indexed(bus: &mut Bus, pc_addr: u32, xy: u16, dbr: u8) -> u32 {
    absolute(bus, pc_addr, dbr) + (xy as u32)
}

/// OPCODE (addr)
pub fn absolute_indexed_indirect(bus: &mut Bus, pc_addr: u32, xy: u16, dbr: u8) -> u32 {
    let addr = absolute_indexed(bus, pc_addr, xy, dbr);
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
pub fn stack_relative(bus: &mut Bus, pc_addr: u32, stack_pointer: u16, dbr: u8) -> u32 {
    absolute_indexed(bus, pc_addr, stack_pointer, dbr)
}

/// OPCODE (sr,S),X
/// OPCODE (sr,S),Y
pub fn stack_relative_indirect_indexed(bus: &mut Bus, pc_addr: u32, stack_pointer: u16, xy: u16, dbr: u8) -> u32 {
    absolute_indexed(bus, pc_addr, stack_pointer, dbr) + (xy as u32)
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
    pub fn effective_address(self, bus: &mut Bus, params: ResolveAddressParams) -> u32 {
        use IndexRegister::X as X;
        let p = params;
        // TODO: maybe use impl Immediate {pub fn effective_address} to prevent this match statement?
        match self {
            Self::Accumulator => p.pc_addr,
            Self::Immediate => immediate(p.pc_addr),
            Self::Absolute => absolute(bus, p.pc_addr, p.dbr),
            Self::AbsoluteIndirect => absolute_indirect(bus, p.pc_addr, p.dbr),
            Self::AbsoluteIndirectLong => absolute_indirect_long(bus, p.pc_addr, p.dbr),
            Self::AbsoluteLong => absolute_long(bus, p.pc_addr),
            Self::DirectPage => direct_page(bus, p.pc_addr, p.direct_page_register),
            Self::DirectPageIndirect => direct_page_indirect(bus, p.pc_addr, p.direct_page_register),
            Self::DirectPageIndirectLong => direct_page_indirect_long(bus, p.pc_addr, p.direct_page_register),
            Self::AbsoluteIndexed(idx) => absolute_indexed(bus, p.pc_addr, if idx == X {p.x} else {p.y}, p.dbr),
            Self::AbsoluteIndexedIndirect(idx) => absolute_indexed_indirect(bus, p.pc_addr, if idx == X {p.x} else {p.y}, p.dbr),
            Self::AbsoluteLongIndexed(idx) => absolute_long_indexed(bus, p.pc_addr, if idx == X {p.x} else {p.y}),
            Self::DirectPageIndexed(idx) => direct_page_indexed(bus, p.pc_addr, p.direct_page_register, if idx == X {p.x} else {p.y}),
            Self::DirectPageIndexedIndirect(idx) => direct_page_indexed_indirect(bus, p.pc_addr, p.direct_page_register, if idx == X {p.x} else {p.y}),
            Self::DirectPageIndirectIndexed(idx) => direct_page_indirect_indexed(bus, p.pc_addr, p.direct_page_register, if idx == X {p.x} else {p.y}),
            Self::DirectPageIndirectLongIndexed(idx) => direct_page_indirect_long_indexed(bus, p.pc_addr, p.direct_page_register, if idx == X {p.x} else {p.y}),
            Self::StackRelative => stack_relative(bus, p.pc_addr, p.stack_pointer, p.dbr),
            Self::StackRelativeIndirectIndexed(idx) => stack_relative_indirect_indexed(bus, p.pc_addr, p.stack_pointer, if idx == X {p.x} else {p.y}, p.dbr),
        }
    }

    pub fn read_8bit(self, params: ResolveAddressParams, bus: &mut Bus) -> u8 {
        let address = self.effective_address(bus, params);
        bus.read(address)
    }

    pub fn read_16bit(self, params: ResolveAddressParams, bus: &mut Bus) -> u16 {
        let address = self.effective_address(bus, params);
        (bus.read(address) as u16) | ((bus.read(address + 1) as u16) << 8)
    }

    pub fn write_8bit(self, params: ResolveAddressParams, bus: &mut Bus, value: u8) {
        let address = self.effective_address(bus, params);
        bus.write(address, value);
    }

    pub fn write_16bit(self, params: ResolveAddressParams, bus: &mut Bus, value: u16) {
        let address = self.effective_address(bus, params);
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
        assert_eq!(absolute(&mut bus, pc_addr, 0x00), 0x000201);

        let mut bus = Bus::new();
        let pc_addr = 0x7F0000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(absolute(&mut bus, pc_addr, 0x7F), 0x7F0201);
    }

    #[test]
    fn test_absolute_indirect() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let addr = 0x55;
        bus.write(pc_addr + 1, addr);
        bus.write(addr as u32, 0x02);
        bus.write((addr + 1) as u32, 0x01);
        assert_eq!(absolute_indirect(&mut bus, pc_addr, 0x00), 0x000201);

        let pc_addr = 0x7E0010;
        let addr = 0x55;
        bus.write(pc_addr + 1, addr);
        bus.write(addr as u32, 0x02);
        bus.write((addr + 1) as u32, 0x01);
        assert_eq!(absolute_indirect(&mut bus, pc_addr, 0x00), 0x7E0201);

        let pc_addr = 0x7E0010;
        let addr = 0x55;
        bus.write(pc_addr + 1, addr);
        bus.write(addr as u32, 0x02);
        bus.write((addr + 1) as u32, 0x01);
        assert_eq!(absolute_indirect(&mut bus, pc_addr, 0x00), 0x7E0201);
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
        assert_eq!(absolute_indirect_long(&mut bus, pc_addr, 0x00), 0x030201);
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
        assert_eq!(absolute_indexed(&mut bus, pc_addr, 0x02, 0x00), 0x000203);

        let mut bus = Bus::new();
        let pc_addr = 0x7F0000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(absolute_indexed(&mut bus, pc_addr, 0x02, 0x7F), 0x7F0203);
    }

    #[test]
    fn test_absolute_indexed_indirect() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let addr = 0x55;
        bus.write(pc_addr + 1, addr);
        bus.write(addr as u32, 0x02);
        bus.write((addr + 1) as u32, 0x01);
        assert_eq!(absolute_indexed_indirect(&mut bus, pc_addr, 0, 0x00), 0x000201);
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
        assert_eq!(stack_relative(&mut bus, pc_addr, 0x02, 0x00), 0x000203);

        let mut bus = Bus::new();
        let pc_addr = 0x7F0000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(stack_relative(&mut bus, pc_addr, 0x02, 0x7F), 0x7F0203);
    }

    #[test]
    fn test_stack_relative_indirect_indexed() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(stack_relative_indirect_indexed(&mut bus, pc_addr, 0x02, 0x02, 0x00), 0x000205);

        let mut bus = Bus::new();
        let pc_addr = 0x7F0000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(stack_relative_indirect_indexed(&mut bus, pc_addr, 0x02, 0x02, 0x7F), 0x7F0205);
    }

    #[test]
    fn test_address_value() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x20);
        bus.write(pc_addr + 2, 0x10);
        bus.write(0x001020, 0xFE);
        let val = AddressingMode::Absolute.read_8bit(
            ResolveAddressParams {pc_addr, direct_page_register: 0x00, stack_pointer: 0x00, x: 0x00, y: 0x00, dbr: 0x00},
            &mut bus,
        );
        assert_eq!(val, 0xFE);

        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x20);
        bus.write(pc_addr + 2, 0x10);
        bus.write(0x001020, 0xFF);
        bus.write(0x001021, 0xEE);
        let val = AddressingMode::Absolute.read_16bit(
            ResolveAddressParams {pc_addr, direct_page_register: 0x00, stack_pointer: 0x00, x: 0x00, y: 0x00, dbr: 0x00},
            &mut bus,
        );
        assert_eq!(val, 0xEEFF);
    }
}