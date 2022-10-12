use crate::bus::Bus;

/// OPCODE #const
pub fn immediate(pc_addr: u32) -> u32 {
    pc_addr + 1
}

/// OPCODE addr
pub fn absolute(bus: &Bus, pc_addr: u32) -> u32 {
    (pc_addr & 0xFF0000) |
    (bus.read(pc_addr + 1) as u32) |
    ((bus.read(pc_addr + 2) as u32) << 8)
}

/// OPCODE long
pub fn absolute_long(bus: &Bus, pc_addr: u32) -> u32 {
    (bus.read(pc_addr + 1) as u32) |
    ((bus.read(pc_addr + 2) as u32) << 8) |
    ((bus.read(pc_addr + 3) as u32) << 16)
}

/// OPCODE dp
pub fn direct_page(bus: &Bus, pc_addr: u32, direct_page_register: u16) -> u32 {
    (bus.read(pc_addr + 1) as u32) + direct_page_register as u32
}

/// OPCODE (dp)
pub fn direct_page_indirect(bus: &Bus, pc_addr: u32, direct_page_register: u16) -> u32 {
    let addr = direct_page(bus, pc_addr, direct_page_register);
    let dbr = pc_addr & 0xFF0000;
    dbr | ((bus.read(addr) as u32) << 8) | (bus.read(addr + 1) as u32)
}

/// OPCODE [dp]
pub fn direct_page_indirect_long(bus: &Bus, pc_addr: u32, direct_page_register: u16) -> u32 {
    let addr = direct_page(bus, pc_addr, direct_page_register);
    ((bus.read(addr) as u32) << 16) |
    ((bus.read(addr + 1) as u32) << 8) |
    (bus.read(addr + 2) as u32)
}

/// OPCODE addr,X
/// OPCODE addr,Y
pub fn absolute_indexed(bus: &Bus, pc_addr: u32, xy: u16) -> u32 {
    absolute(bus, pc_addr) + (xy as u32)
}

/// OPCODE long,X
/// OPCODE long,Y
pub fn absolute_long_indexed(bus: &Bus, pc_addr: u32, xy: u16) -> u32 {
    absolute_long(bus, pc_addr) + (xy as u32)
}

/// OPCODE dp,X
/// OPCODE dp,Y
pub fn direct_page_indexed(bus: &Bus, pc_addr: u32, direct_page_register: u16, xy: u16) -> u32 {
    direct_page(bus, pc_addr, direct_page_register) + (xy as u32)
}

/// OPCODE (dp,X)
/// OPCODE (dp,Y)
pub fn direct_page_indexed_indirect(bus: &Bus, pc_addr: u32, direct_page_register: u16, xy: u16) -> u32 {
    direct_page_indirect(bus, pc_addr, direct_page_register + xy)
}

/// OPCODE (dp),X
/// OPCODE (dp),Y
pub fn direct_page_indirect_indexed(bus: &Bus, pc_addr: u32, direct_page_register: u16, xy: u16) -> u32 {
    direct_page_indirect(bus, pc_addr, direct_page_register) + (xy as u32)
}

/// OPCODE [dp],X
/// OPCODE [dp],Y
pub fn direct_page_indirect_long_indexed(bus: &Bus, pc_addr: u32, direct_page_register: u16, xy: u16) -> u32 {
    direct_page_indirect_long(bus, pc_addr, direct_page_register) + (xy as u32)
}

/// OPCODE sr,S
pub fn stack_relative(bus: &Bus, pc_addr: u32, stack_pointer: u16) -> u32 {
    absolute_indexed(bus, pc_addr, stack_pointer)
}

/// OPCODE (sr,S),X
/// OPCODE (sr,S),Y
pub fn stack_relative_indirect_indexed(bus: &Bus, pc_addr: u32, stack_pointer: u16, xy: u16) -> u32 {
    absolute_indexed(bus, pc_addr, stack_pointer) + (xy as u32)
}

#[derive(Copy, Clone)]
pub enum AddressingMode {
    Immediate,
    Absolute,
    AbsoluteLong,
    DirectPage,
    DirectPageIndirect,
    DirectPageIndirectLong,
    AbsoluteIndexed,
    AbsoluteLongIndexed,
    DirectPageIndexed,
    DirectPageIndexedIndirect,
    DirectPageIndirectIndexed,
    DirectPageIndirectLongIndexed,
    StackRelative,
    StackRelativeIndirectIndexed,
}

impl AddressingMode {
    pub fn effective_address(self, bus: &Bus, pc_addr: u32, direct_page_register: u16, stack_pointer: u16, xy: u16) -> u32 {
        match self {
            AddressingMode::Immediate => immediate(pc_addr),
            AddressingMode::Absolute => absolute(bus, pc_addr),
            AddressingMode::AbsoluteLong => absolute_long(bus, pc_addr),
            AddressingMode::DirectPage => direct_page(bus, pc_addr, direct_page_register),
            AddressingMode::DirectPageIndirect => direct_page_indirect(bus, pc_addr, direct_page_register),
            AddressingMode::DirectPageIndirectLong => direct_page_indirect_long(bus, pc_addr, direct_page_register),
            AddressingMode::AbsoluteIndexed => absolute_indexed(bus, pc_addr, xy),
            AddressingMode::AbsoluteLongIndexed => absolute_long_indexed(bus, pc_addr, xy),
            AddressingMode::DirectPageIndexed => direct_page_indexed(bus, pc_addr, direct_page_register, xy),
            AddressingMode::DirectPageIndexedIndirect => direct_page_indexed_indirect(bus, pc_addr, direct_page_register, xy),
            AddressingMode::DirectPageIndirectIndexed => direct_page_indirect_indexed(bus, pc_addr, direct_page_register, xy),
            AddressingMode::DirectPageIndirectLongIndexed => direct_page_indirect_long_indexed(bus, pc_addr, direct_page_register, xy),
            AddressingMode::StackRelative => stack_relative(bus, pc_addr, stack_pointer),
            AddressingMode::StackRelativeIndirectIndexed => stack_relative_indirect_indexed(bus, pc_addr, stack_pointer, xy),
        }
    }

    pub fn value_8bit(self, bus: &Bus, pc_addr: u32, direct_page_register: u16, stack_pointer: u16, xy: u16) -> u8 {
        let address = self.effective_address(bus, pc_addr, direct_page_register, stack_pointer, xy);
        return bus.read(address);
    }

    pub fn value_16bit(self, bus: &Bus, pc_addr: u32, direct_page_register: u16, stack_pointer: u16, xy: u16) -> u16 {
        let address = self.effective_address(bus, pc_addr, direct_page_register, stack_pointer, xy);
        return (bus.read(address) as u16) | ((bus.read(address + 1) as u16) << 8);
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
        assert_eq!(absolute(&bus, pc_addr), 0x000201);

        let mut bus = Bus::new();
        let pc_addr = 0x7F0000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(absolute(&bus, pc_addr), 0x7F0201);
    }

    #[test]
    fn test_absolute_long() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        bus.write(pc_addr + 3, 0x03);
        assert_eq!(absolute_long(&bus, pc_addr), 0x030201);

        let mut bus = Bus::new();
        let pc_addr = 0x7E0010;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        bus.write(pc_addr + 3, 0x03);
        assert_eq!(absolute_long(&bus, pc_addr), 0x030201);
    }

    #[test]
    fn test_direct_page() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        assert_eq!(direct_page(&bus, pc_addr, 0x00), 0x000055);

        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        assert_eq!(direct_page(&bus, pc_addr, 0x01), 0x000056);
    }

    #[test]
    fn test_direct_page_indirect() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write(dp as u32, 0x02);
        bus.write((dp + 1) as u32, 0x01);
        assert_eq!(direct_page_indirect(&bus, pc_addr, 0x00), 0x000201);

        let pc_addr = 0x7E0010;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write(dp as u32, 0x02);
        bus.write((dp + 1) as u32, 0x01);
        assert_eq!(direct_page_indirect(&bus, pc_addr, 0x00), 0x7E0201);

        let pc_addr = 0x7E0010;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write((dp + 1) as u32, 0x02);
        bus.write((dp + 2) as u32, 0x01);
        assert_eq!(direct_page_indirect(&bus, pc_addr, 0x01), 0x7E0201);
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
        assert_eq!(direct_page_indirect_long(&bus, pc_addr, 0x00), 0x030201);

        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write((dp + 1) as u32, 0x03);
        bus.write((dp + 2) as u32, 0x02);
        bus.write((dp + 3) as u32, 0x01);
        assert_eq!(direct_page_indirect_long(&bus, pc_addr, 0x01), 0x030201);
    }

    #[test]
    fn test_absolute_indexed() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(absolute_indexed(&bus, pc_addr, 0x02), 0x000203);

        let mut bus = Bus::new();
        let pc_addr = 0x7F0000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(absolute_indexed(&bus, pc_addr, 0x02), 0x7F0203);
    }

    #[test]
    fn test_absolute_long_indexed() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        bus.write(pc_addr + 3, 0x03);
        assert_eq!(absolute_long_indexed(&bus, pc_addr, 0x02), 0x030203);

        let mut bus = Bus::new();
        let pc_addr = 0x7E0010;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        bus.write(pc_addr + 3, 0x03);
        assert_eq!(absolute_long_indexed(&bus, pc_addr, 0x02), 0x030203);
    }

    #[test]
    fn test_direct_page_indexed() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        assert_eq!(direct_page_indexed(&bus, pc_addr, 0x00, 0x01), 0x000056);

        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        assert_eq!(direct_page_indexed(&bus, pc_addr, 0x01, 0x01), 0x000057);
    }

    #[test]
    fn test_direct_page_indexed_indirect() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write((dp + 1) as u32, 0x02);
        bus.write((dp + 2) as u32, 0x01);
        assert_eq!(direct_page_indexed_indirect(&bus, pc_addr, 0x00, 0x01), 0x000201);

        let pc_addr = 0x7E0010;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write((dp + 1) as u32, 0x02);
        bus.write((dp + 2) as u32, 0x01);
        assert_eq!(direct_page_indexed_indirect(&bus, pc_addr, 0x00, 0x01), 0x7E0201);

        let pc_addr = 0x7E0010;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write((dp + 2) as u32, 0x02);
        bus.write((dp + 3) as u32, 0x01);
        assert_eq!(direct_page_indexed_indirect(&bus, pc_addr, 0x01, 0x01), 0x7E0201);
    }

    #[test]
    fn test_direct_page_indirect_indexed() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write(dp as u32, 0x02);
        bus.write((dp + 1) as u32, 0x01);
        assert_eq!(direct_page_indirect_indexed(&bus, pc_addr, 0x00, 0x01), 0x000202);

        let pc_addr = 0x7E0010;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write(dp as u32, 0x02);
        bus.write((dp + 1) as u32, 0x01);
        assert_eq!(direct_page_indirect_indexed(&bus, pc_addr, 0x00, 0x01), 0x7E0202);

        let pc_addr = 0x7E0010;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write((dp + 1) as u32, 0x02);
        bus.write((dp + 2) as u32, 0x01);
        assert_eq!(direct_page_indirect_indexed(&bus, pc_addr, 0x01, 0x01), 0x7E0202);
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
        assert_eq!(direct_page_indirect_long_indexed(&bus, pc_addr, 0x00, 0x02), 0x030203);

        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        let dp = 0x55;
        bus.write(pc_addr + 1, dp);
        bus.write((dp + 1) as u32, 0x03);
        bus.write((dp + 2) as u32, 0x02);
        bus.write((dp + 3) as u32, 0x01);
        assert_eq!(direct_page_indirect_long_indexed(&bus, pc_addr, 0x01, 0x02), 0x030203);
    }

    #[test]
    fn test_stack_relative() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(stack_relative(&bus, pc_addr, 0x02), 0x000203);

        let mut bus = Bus::new();
        let pc_addr = 0x7F0000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(stack_relative(&bus, pc_addr, 0x02), 0x7F0203);
    }

    #[test]
    fn test_stack_relative_indirect_indexed() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(stack_relative_indirect_indexed(&bus, pc_addr, 0x02, 0x02), 0x000205);

        let mut bus = Bus::new();
        let pc_addr = 0x7F0000;
        bus.write(pc_addr + 1, 0x01);
        bus.write(pc_addr + 2, 0x02);
        assert_eq!(stack_relative_indirect_indexed(&bus, pc_addr, 0x02, 0x02), 0x7F0205);
    }

    #[test]
    fn test_address_value() {
        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x20);
        bus.write(pc_addr + 2, 0x10);
        bus.write(0x001020, 0xFE);
        let val = AddressingMode::Absolute.value_8bit(&bus, pc_addr, 0x00, 0x00, 0x00);
        assert_eq!(val, 0xFE);

        let mut bus = Bus::new();
        let pc_addr = 0x000000;
        bus.write(pc_addr + 1, 0x20);
        bus.write(pc_addr + 2, 0x10);
        bus.write(0x001020, 0xFF);
        bus.write(0x001021, 0xEE);
        let val = AddressingMode::Absolute.value_16bit(&bus, pc_addr, 0x00, 0x00, 0x00);
        assert_eq!(val, 0xEEFF);
    }
}