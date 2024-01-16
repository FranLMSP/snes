pub const MDMAEN: u16       = 0x420B;  // Select General Purpose DMA Channel(s) and Start Transfer (W)

pub const DMAPX: u16        = 0x4300;  // DMA/HDMA Parameters (R/W)
pub const BBADX: u16        = 0x4301;  // DMA/HDMA I/O-Bus Address (PPU-Bus aka B-Bus) (R/W)
pub const A1TXL: u16        = 0x4302;  // HDMA Table Start Address (low) / DMA Current Addr (low) (R/W)
pub const A1TXH: u16        = 0x4303;  // HDMA Table Start Address (hi) / DMA Current Addr (hi) (R/W)
pub const A1BX:  u16        = 0x4304;  // HDMA Table Start Address (bank) / DMA Current Addr (bank) (R/W)

pub const DASXL: u16        = 0x4305;  // Indirect HDMA Address (low) / DMA Byte-Counter (low) (R/W)
pub const DASXH: u16        = 0x4306;  // Indirect HDMA Address (hi) / DMA Byte-Counter (hi) (R/W)
pub const DASBX: u16        = 0x4307;  // Indirect HDMA Address (bank) (R/W)

#[derive(PartialEq, Debug)]
pub enum DMAChannel {
    Channel0,
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
}

pub enum TransferDirection {
    AtoB,
    BtoA,
}

pub enum AutoUpdateA {
    Increment,
    Decrement,
    Neither,
}

pub enum DMAAddressingMode {
    Direct,
    Indirect,
}

pub enum TransferFormat {
    OneByteOneRegister,     // Write once
    TwoBytesTwoRegisters,   // Write once
    TwoBytesOneRegister,    // Write twice
    FourBytesTwoRegisters,  // Write twice
    FourBytesTwoRegistersSequential,  // Write twice
    FourBytesFourRegisters, // Write once
}

#[allow(dead_code)] pub struct DMATransferProps {
    channel: DMAChannel,
    channel_number: u8,
    direction: TransferDirection,
    addressing_mode: DMAAddressingMode,
    auto_update_a_address: AutoUpdateA,
    transfer_format: TransferFormat,
    a_bus_address: u32,
    b_bus_address: u8,
    number_of_bytes: u16,
}

impl DMATransferProps {
    fn read_channel_register(registers: &[u8], register: u16, channel_number: u8) -> u8 {
        registers[(((register & 0xFF0F) | ((channel_number as u16) << 4)) - 0x4300) as usize]
    }

    fn write_channel_register(registers: &mut [u8], register: u16, channel_number: u8, val: u8) {
        registers[(((register & 0xFF0F) | ((channel_number as u16) << 4)) - 0x4300) as usize] = val;
    }

    pub fn new_from_registers(
        registers: &[u8],
        channel_number: u8,
    ) -> Self {
        let channel = match channel_number {
            0 => DMAChannel::Channel0,
            1 => DMAChannel::Channel1,
            2 => DMAChannel::Channel2,
            3 => DMAChannel::Channel3,
            4 => DMAChannel::Channel4,
            5 => DMAChannel::Channel5,
            6 => DMAChannel::Channel6,
            7 => DMAChannel::Channel7,
            _ => unreachable!(),
        };
        let params = DMATransferProps::read_channel_register(registers, DMAPX, channel_number);
        let direction = match params >> 7 == 1 {
            false => TransferDirection::AtoB,
            true => TransferDirection::BtoA,
        };
        let addressing_mode= match (params >> 6) & 1 == 1 {
            false => DMAAddressingMode::Direct,
            true => DMAAddressingMode::Indirect,
        };
        let auto_update_a_address = match (params >> 3) & 0b11 {
            0b00 => AutoUpdateA::Increment,
            0b10 => AutoUpdateA::Decrement,
            0b01 | 0b11 => AutoUpdateA::Neither,
            _ => unreachable!(),
        };
        let transfer_format = match params & 0b111 {
            0 => TransferFormat::OneByteOneRegister,
            1 => TransferFormat::TwoBytesTwoRegisters,
            2 => TransferFormat::TwoBytesOneRegister,
            3 => TransferFormat::FourBytesTwoRegisters,
            4 => TransferFormat::FourBytesFourRegisters,
            5 => TransferFormat::FourBytesTwoRegistersSequential,
            6 => TransferFormat::TwoBytesOneRegister,
            7 => TransferFormat::FourBytesTwoRegisters,
            _ => unreachable!(),
        };
        let a_bus_address =
            (DMATransferProps::read_channel_register(registers, A1BX, channel_number) as u32) << 16 |
            (DMATransferProps::read_channel_register(registers, A1TXH, channel_number) as u32) << 8 |
             DMATransferProps::read_channel_register(registers, A1TXL, channel_number) as u32;
        let b_bus_address = DMATransferProps::read_channel_register(registers, BBADX, channel_number);
        let number_of_bytes =
            (DMATransferProps::read_channel_register(registers, DASXH, channel_number) as u16) << 8 |
            DMATransferProps::read_channel_register(registers, DASXL, channel_number) as u16;

        Self {
            channel,
            channel_number,
            direction,
            addressing_mode,
            auto_update_a_address,
            transfer_format,
            a_bus_address,
            b_bus_address,
            number_of_bytes,
        }
    }

    fn save_channel_state(&self, registers: &mut [u8]) {
        // Update B Bus address
        DMATransferProps::write_channel_register(registers, BBADX, self.channel_number, self.b_bus_address);
        // Update A Bus address
        DMATransferProps::write_channel_register(registers, A1BX, self.channel_number, (self.a_bus_address >> 16) as u8);
        DMATransferProps::write_channel_register(registers, A1TXH, self.channel_number, (self.a_bus_address >> 8) as u8);
        DMATransferProps::write_channel_register(registers, A1BX, self.channel_number, self.a_bus_address as u8);
        // Update Byte count
        DMATransferProps::write_channel_register(registers, DASXH, self.channel_number, (self.number_of_bytes >> 8) as u8);
        DMATransferProps::write_channel_register(registers, DASXL, self.channel_number, self.number_of_bytes as u8);
    }

    pub fn tick(&mut self, registers: &mut [u8]) -> Vec<(u32, u32)> {
        let source_address = match self.direction {
            TransferDirection::AtoB => self.a_bus_address,
            TransferDirection::BtoA => 0x002100 | (self.b_bus_address as u32),
        };
        let dest_address = match self.direction {
            TransferDirection::AtoB => 0x002100 | (self.b_bus_address as u32),
            TransferDirection::BtoA => self.a_bus_address,
        };

        let mut pending_bus_writes = Vec::new();
        match self.transfer_format {
            TransferFormat::OneByteOneRegister => {
                // Transfer 1 byte    xx                    ;eg. for WRAM (port 2180h)
                pending_bus_writes.push((source_address, dest_address));
            },
            TransferFormat::TwoBytesTwoRegisters => {
                // Transfer 2 bytes   xx, xx+1              ;eg. for VRAM (port 2118h/19h)
                for index in 0..2 {
                    pending_bus_writes.push((
                        source_address.wrapping_add(index),
                        dest_address.wrapping_add(index),
                    ));
                }
            },
            TransferFormat::TwoBytesOneRegister => {
                // Transfer 2 bytes   xx, xx                ;eg. for OAM or CGRAM
                for index in 0..2 {
                    pending_bus_writes.push((
                        source_address.wrapping_add(index),
                        dest_address,
                    ));
                }
            },
            TransferFormat::FourBytesTwoRegisters => {
                // Transfer 4 bytes   xx, xx,   xx+1, xx+1  ;eg. for BGnxOFS, M7x
                for index in 0..4 {
                    pending_bus_writes.push((
                        source_address.wrapping_add(index),
                        dest_address.wrapping_add(index / 2),
                    ));
                }
            },
            TransferFormat::FourBytesFourRegisters => {
                // Transfer 4 bytes   xx, xx+1, xx+2, xx+3  ;eg. for BGnSC, Window, APU..
                for index in 0..4 {
                    pending_bus_writes.push((
                        source_address.wrapping_add(index),
                        dest_address.wrapping_add(index),
                    ));
                }
            },
            TransferFormat::FourBytesTwoRegistersSequential => {
                // Transfer 4 bytes   xx, xx+1, xx,   xx+1  ;whatever purpose, VRAM maybe
                for index in 0..4 {
                    pending_bus_writes.push((
                        source_address.wrapping_add(index),
                        dest_address.wrapping_add(index & 1),
                    ));
                }
            },
        };

        if self.number_of_bytes > 0 {
            self.number_of_bytes -= 1;
        }
        match self.auto_update_a_address {
            AutoUpdateA::Increment => self.a_bus_address = self.a_bus_address.wrapping_add(1),
            AutoUpdateA::Decrement => self.a_bus_address = self.a_bus_address.wrapping_sub(1),
            AutoUpdateA::Neither => {},
        }

        self.save_channel_state(registers);
        pending_bus_writes
    }
}


pub struct DMA {
    pub active_dma_transfers: Vec<DMATransferProps>,
    registers: [u8; 0x1D00],
}

impl DMA {
    pub fn new() -> Self {
        Self {
            active_dma_transfers: Vec::new(),
            registers: [0; 0x1D00],
        }
    }

    fn _read(&self, address: u16) -> u8 {
        self.registers[(address - 0x4300) as usize]
    }

    fn _write(&mut self, address: u16, value: u8) {
        self.registers[(address - 0x4300) as usize] = value;
    }

    pub fn prepare_dma_transfer(&mut self, dma_select: u8) {
        self.active_dma_transfers = Vec::new();
        let mut active_channels = [false; 8];
        for (i, active_channel) in active_channels.iter_mut().enumerate() {
            *active_channel = (dma_select & (1 << i)) != 0;
        }

        for (channel, is_active) in active_channels.iter().enumerate() {
            if !is_active {
                continue;
            }
            let props = DMATransferProps::new_from_registers(
                &self.registers,
                channel as u8,
            );
            self.active_dma_transfers.push(props);
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self._read(address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self._write(address, value);
    }

    pub fn is_active(&self) -> bool {
        !self.active_dma_transfers.is_empty()
    }

    pub fn tick(&mut self) -> Vec<(u32, u32)> {
        if !self.is_active() {
            return vec![];
        }

        let pending_bus_writes = self.active_dma_transfers[0].tick(&mut self.registers);
        if self.active_dma_transfers[0].number_of_bytes == 0 {
            self.active_dma_transfers.remove(0);
        }
        pending_bus_writes
    }
}

impl Default for DMA {
    fn default() -> Self {
        Self::new()
    }
}
