use crate::cpu::CPU;
use crate::cpu::bus::Bus;
use crate::rom::ROM;
use crate::rom::lo_rom::LoROM;

pub struct Emulator {
    pub cpu: CPU,
    pub bus: Bus,
    pub rom: Box<dyn ROM>,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            bus: Bus::new(),
            rom: Box::new(LoROM::new()),
        }
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus);
        self.bus.ppu.tick(self.cpu.cycles);

        self.cpu.cycles = 0;
    }

    pub fn loop_frame(&mut self) {
        let mut frame_started = true;
        loop {
            self.tick();
            if !frame_started && self.bus.ppu.registers.v_count == 260 {
                break;
            }
            if self.bus.ppu.registers.v_count >= 261 {
                frame_started = false;
            }
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset_vector(&mut self.bus);
    }
}