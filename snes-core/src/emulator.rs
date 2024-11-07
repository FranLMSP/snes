use crate::cpu::CPU;
use crate::cpu::bus::Bus;

pub struct Emulator {
    pub cpu: CPU,
    pub bus: Bus,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            bus: Bus::new(),
        }
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus);
        self.bus.ppu.tick(self.cpu.registers.cycles);

        self.cpu.registers.cycles = 0;
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

    pub fn reset_vector(&mut self) {
        self.cpu.reset_vector(&mut self.bus);
    }

    pub fn hard_reset(&mut self) {
        self.cpu = CPU::new();
        self.bus.hard_reset();
        self.reset_vector();
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}