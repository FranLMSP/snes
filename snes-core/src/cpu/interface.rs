use super::{bus::Bus, cycles, dma, instructions::{mapper::map_opcode_to_instruction, move_common}, registers::Registers};

pub struct CPU {
    pub registers: Registers,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }

    fn check_running_state(&mut self, bus: &mut Bus) -> bool {
        // Each byte in a DMA transfer takes 8 master cycles.
        // And each CPU can take either 6, 8 or 12 master cycles depending
        // on what's being read from memory. So this won't be accurate.
        if bus.dma.is_active() {
            let pending_bus_writes = bus.dma.tick();
            for (src, dst) in pending_bus_writes {
                let byte = bus.read(src);
                bus.write(dst, byte);
                let (bytes, cycles) = cycles::increment_cycles_while_stopped();
                self.registers.increment_pc(bytes); self.registers.cycles += cycles;
            }
            if !bus.dma.is_active() {
                bus.write(dma::MDMAEN as u32, 0x00)
            }
            return false;
        }
        if self.registers.is_cpu_stopped {
            let (bytes, cycles) = cycles::increment_cycles_while_stopped();
            self.registers.increment_pc(bytes); self.registers.cycles += cycles;
            return false;
        }
        if self.registers.is_cpu_waiting_interrupt {
            // TODO: check for interrupts here
            let (bytes, cycles) = cycles::increment_cycles_while_stopped();
            self.registers.increment_pc(bytes); self.registers.cycles += cycles;
            return false;
        }
        if self.registers.is_moving {
            let is_next = self.registers.is_move_next;
            move_common::tick_move(&mut self.registers, bus, is_next);
            return false;
        }
        true
    }

    pub fn tick(&mut self, bus: &mut Bus) {
        if !self.check_running_state(bus) {
            return;
        }
        let opcode = bus.read(self.registers.get_pc_address());
        let instruction = map_opcode_to_instruction(opcode);
        instruction.execute(&mut self.registers, bus);
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}
