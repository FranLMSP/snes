extern crate snes_core;
use snes_core::{emulator::Emulator, cpu::instructions::mapper::map_opcode_to_instruction};

pub struct CPUDisassembler {
    _history_limit: usize,
    _instruction_history: Vec<String>,
}

impl CPUDisassembler {
    pub fn get_next_instruction(emulator: &Emulator) -> String {
        let opcode = emulator.bus.read_external(emulator.cpu.registers.get_pc_address());
        let instruction = map_opcode_to_instruction(opcode);
        instruction.mnemonic(&emulator.cpu.registers, &emulator.bus, opcode)
    }
}
