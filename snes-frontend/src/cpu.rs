extern crate snes_core;
use snes_core::emulator::Emulator;

pub struct CPUDisassembler {
    _history_limit: usize,
    _instruction_history: Vec<String>,
}

impl CPUDisassembler {
    pub fn get_next_instruction(emulator: &Emulator) -> String {
        let opcode = emulator.bus.read(emulator.cpu.registers.get_pc_address());
        let is_cpu_16bit = emulator.cpu.registers.is_16bit_mode();
        let is_index_16bit = emulator.cpu.registers.is_16bit_index();
        let next_byte = emulator.bus.read(emulator.cpu.registers.get_pc_address() + 1);
        let next_second_byte = emulator.bus.read(emulator.cpu.registers.get_pc_address() + 2);
        let next_third_byte = emulator.bus.read(emulator.cpu.registers.get_pc_address() + 3);
        let next_word = {
            (emulator.bus.read(emulator.cpu.registers.get_pc_address() + 1) as u16) |
            ((emulator.bus.read(emulator.cpu.registers.get_pc_address() + 2) as u16) << 8)
        };
        let next_word_long = {
            (emulator.bus.read(emulator.cpu.registers.get_pc_address() + 1) as u32) |
            ((emulator.bus.read(emulator.cpu.registers.get_pc_address() + 2) as u32) << 8) |
            ((emulator.bus.read(emulator.cpu.registers.get_pc_address() + 3) as u32) << 16)
        };
        match opcode {
            // ADC
            0x69 => match is_cpu_16bit {
                true =>   format!("{:02X} {:02X} {:02X} __ | ADC #${:04X}", opcode, next_byte, next_second_byte, next_word),
                false =>  format!("{:02X} {:02X} __ __ | ADC #${:02X}", opcode, next_byte, next_byte),
            },
            0x6D => format!("{:02X} {:02X} {:02X} __ | ADC ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x6F => format!("{:02X} {:02X} {:02X} {:02X} | ADC ${:06X}", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0x65 => format!("{:02X} {:02X} __ __ | ADC ${:02X} | dp", opcode, next_byte, next_byte),
            0x72 => format!("{:02X} {:02X} __ __ | ADC (${:02X})", opcode, next_byte, next_byte),
            0x67 => format!("{:02X} {:02X} __ __ | ADC [${:02X}]", opcode, next_byte, next_byte),
            0x7D => format!("{:02X} {:02X} {:02X} __ | ADC ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0x7F => format!("{:02X} {:02X} {:02X} {:02X} | ADC ${:06X}, X", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0x79 => format!("{:02X} {:02X} {:02X} __ | ADC ${:04X}, Y", opcode, next_byte, next_second_byte, next_word),
            0x75 => format!("{:02X} {:02X} __ __ | ADC ${:02X}, X | dp", opcode, next_byte, next_byte),
            0x61 => format!("{:02X} {:02X} __ __ | ADC (${:02X}, X) | dp", opcode, next_byte, next_byte),
            0x71 => format!("{:02X} {:02X} __ __ | ADC (${:02X}), Y | dp", opcode, next_byte, next_byte),
            0x77 => format!("{:02X} {:02X} __ __ | ADC [${:02X}], Y | dp", opcode, next_byte, next_byte),
            0x63 => format!("{:02X} {:02X} __ __ | ADC {:02X}, S | sr", opcode, next_byte, next_byte),
            0x73 => format!("{:02X} {:02X} __ __ | ADC ({:02X}, S), Y | sr", opcode, next_byte, next_byte),
            // AND
            0x29 => match is_cpu_16bit {
                true =>   format!("{:02X} {:02X} {:02X} __ | AND #${:04X}", opcode, next_byte, next_second_byte, next_word),
                false =>  format!("{:02X} {:02X} __ __ | AND #${:02X}", opcode, next_byte, next_byte),
            },
            0x2D => format!("{:02X} {:02X} {:02X} __ | AND ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x2F => format!("{:02X} {:02X} {:02X} {:02X} | AND ${:06X}", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0x25 => format!("{:02X} {:02X} __ __ | AND ${:02X} | dp", opcode, next_byte, next_byte),
            0x32 => format!("{:02X} {:02X} __ __ | AND (${:02X})", opcode, next_byte, next_byte),
            0x27 => format!("{:02X} {:02X} __ __ | AND [${:02X}]", opcode, next_byte, next_byte),
            0x3D => format!("{:02X} {:02X} {:02X} __ | AND ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0x3F => format!("{:02X} {:02X} {:02X} {:02X} | AND ${:06X}, X", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0x39 => format!("{:02X} {:02X} {:02X} __ | AND ${:04X}, Y", opcode, next_byte, next_second_byte, next_word),
            0x35 => format!("{:02X} {:02X} __ __ | AND ${:02X}, X | dp", opcode, next_byte, next_byte),
            0x21 => format!("{:02X} {:02X} __ __ | AND (${:02X}, X) | dp", opcode, next_byte, next_byte),
            0x31 => format!("{:02X} {:02X} __ __ | AND (${:02X}), Y | dp", opcode, next_byte, next_byte),
            0x37 => format!("{:02X} {:02X} __ __ | AND [${:02X}], Y | dp", opcode, next_byte, next_byte),
            0x23 => format!("{:02X} {:02X} __ __ | AND {:02X}, S | sr", opcode, next_byte, next_byte),
            0x33 => format!("{:02X} {:02X} __ __ | AND ({:02X}, S), Y | sr", opcode, next_byte, next_byte),
            // ASL
            0x0A => format!("{:02X} __ __ __ | ASL A", opcode),
            0x0E => format!("{:02X} {:02X} {:02X} __ | ASL ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x06 => format!("{:02X} {:02X} __ __ | ASL ${:02X} | dp", opcode, next_byte, next_byte),
            0x1E => format!("{:02X} {:02X} {:02X} __ | ASL ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0x16 => format!("{:02X} {:02X} __ __ | ASL ${:02X}, X | dp", opcode, next_byte, next_byte),
            // BCC
            0x90 => format!("{:02X} {:02X} __ __ | BCC ${:02X}", opcode, next_byte, next_byte),
            // BCS
            0xB0 => format!("{:02X} {:02X} __ __ | BCS ${:02X}", opcode, next_byte, next_byte),
            // BEQ
            0xF0 => format!("{:02X} {:02X} __ __ | BEQ ${:02X}", opcode, next_byte, next_byte),
            // BNE
            0xD0 => format!("{:02X} {:02X} __ __ | BNE ${:02X}", opcode, next_byte, next_byte),
            // BMI
            0x30 => format!("{:02X} {:02X} __ __ | BMI ${:02X}", opcode, next_byte, next_byte),
            // BPL
            0x10 => format!("{:02X} {:02X} __ __ | BPL ${:02X}", opcode, next_byte, next_byte),
            // BRA
            0x80 => format!("{:02X} {:02X} __ __ | BRA ${:02X}", opcode, next_byte, next_byte),
            // BRK
            0x00 => format!("{:02X} __ __ __ | BRK", opcode),
            // BRL
            0x82 => format!("{:02X} {:02X} {:02X} __ | BRL ${:04X}", opcode, next_byte, next_second_byte, next_word),
            // BVC
            0x50 => format!("{:02X} {:02X} {:02X} __ | BVC ${:04X}", opcode, next_byte, next_second_byte, next_word),
            // BVS
            0x70 => format!("{:02X} {:02X} {:02X} __ | BVS ${:04X}", opcode, next_byte, next_second_byte, next_word),
            // BIT
            0x89 => match is_cpu_16bit {
                true =>   format!("{:02X} {:02X} {:02X} __ | BIT #${:04X}", opcode, next_byte, next_second_byte, next_word),
                false =>  format!("{:02X} {:02X} __ __ | BIT #${:02X}", opcode, next_byte, next_byte),
            },
            0x2C => format!("{:02X} {:02X} {:02X} __ | BIT ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x24 => format!("{:02X} {:02X} __ __ | BIT ${:02X} | dp", opcode, next_byte, next_byte),
            0x3C => format!("{:02X} {:02X} {:02X} __ | ADC ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0x34 => format!("{:02X} {:02X} __ __ | AND ${:02X}, X | dp", opcode, next_byte, next_byte),
            // CLC
            0x18 => format!("{:02X} __ __ __ | CLC", opcode),
            // CLD
            0xD8 => format!("{:02X} __ __ __ | CLD", opcode),
            // CLI
            0x58 => format!("{:02X} __ __ __ | CLI", opcode),
            // CLV
            0xB8 => format!("{:02X} __ __ __ | CLV", opcode),
            // CMP
            0xC9 => match is_cpu_16bit {
                true =>   format!("{:02X} {:02X} {:02X} __ | CMP #${:04X}", opcode, next_byte, next_second_byte, next_word),
                false =>  format!("{:02X} {:02X} __ __ | CMP #${:02X}", opcode, next_byte, next_byte),
            },
            0xCD => format!("{:02X} {:02X} {:02X} __ | CMP ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0xCF => format!("{:02X} {:02X} {:02X} {:02X} | CMP ${:06X}", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0xC5 => format!("{:02X} {:02X} __ __ | CMP ${:02X} | dp", opcode, next_byte, next_byte),
            0xD2 => format!("{:02X} {:02X} __ __ | CMP (${:02X})", opcode, next_byte, next_byte),
            0xC7 => format!("{:02X} {:02X} __ __ | CMP [${:02X}]", opcode, next_byte, next_byte),
            0xDD => format!("{:02X} {:02X} {:02X} __ | CMP ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0xDF => format!("{:02X} {:02X} {:02X} {:02X} | CMP ${:06X}, X", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0xD9 => format!("{:02X} {:02X} {:02X} __ | CMP ${:04X}, Y", opcode, next_byte, next_second_byte, next_word),
            0xD5 => format!("{:02X} {:02X} __ __ | CMP ${:02X}, X | dp", opcode, next_byte, next_byte),
            0xC1 => format!("{:02X} {:02X} __ __ | CMP (${:02X}, X) | dp", opcode, next_byte, next_byte),
            0xD1 => format!("{:02X} {:02X} __ __ | CMP (${:02X}), Y | dp", opcode, next_byte, next_byte),
            0xD7 => format!("{:02X} {:02X} __ __ | CMP [${:02X}], Y | dp", opcode, next_byte, next_byte),
            0xC3 => format!("{:02X} {:02X} __ __ | CMP {:02X}, S | sr", opcode, next_byte, next_byte),
            0xD3 => format!("{:02X} {:02X} __ __ | CMP ({:02X}, S), Y | sr", opcode, next_byte, next_byte),
            // COP
            0x02 => match is_index_16bit {
                true =>   format!("{:02X} {:02X} {:02X} __ | COP #${:04X}", opcode, next_byte, next_second_byte, next_word),
                false =>  format!("{:02X} {:02X} __ __ | COP #${:02X}", opcode, next_byte, next_byte),
            },
            // CPX
            0xE0 => match is_index_16bit {
                true =>   format!("{:02X} {:02X} {:02X} __ | CPX #${:04X}", opcode, next_byte, next_second_byte, next_word),
                false =>  format!("{:02X} {:02X} __ __ | CPX #${:02X}", opcode, next_byte, next_byte),
            },
            0xEC => format!("{:02X} {:02X} {:02X} __ | CPX ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0xE4 => format!("{:02X} {:02X} __ __ | CPX ${:02X} | dp", opcode, next_byte, next_byte),
            // CPY
            0xC0 => match is_index_16bit {
                true =>   format!("{:02X} {:02X} {:02X} __ | CPY #${:04X}", opcode, next_byte, next_second_byte, next_word),
                false =>  format!("{:02X} {:02X} __ __ | CPY #${:02X}", opcode, next_byte, next_byte),
            },
            0xCC => format!("{:02X} {:02X} {:02X} __ | CPY ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0xC4 => format!("{:02X} {:02X} __ __ | CPY ${:02X} | dp", opcode, next_byte, next_byte),
            // DEC
            0x3A => format!("{:02X} __ __ __ | DEC A", opcode),
            0xCE => format!("{:02X} {:02X} {:02X} __ | DEC ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0xC6 => format!("{:02X} {:02X} __ __ | DEC ${:02X} | dp", opcode, next_byte, next_byte),
            0xDE => format!("{:02X} {:02X} {:02X} __ | DEC ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0xD6 => format!("{:02X} {:02X} __ __ | DEC ${:02X}, X | dp", opcode, next_byte, next_byte),
            // DEX
            0xCA => format!("{:02X} __ __ __ | DEX", opcode),
            // DEY
            0x88 => format!("{:02X} __ __ __ | DEY", opcode),
            // EOR
            0x49 => match is_cpu_16bit {
                true =>   format!("{:02X} {:02X} {:02X} __ | EOR #${:04X}", opcode, next_byte, next_second_byte, next_word),
                false =>  format!("{:02X} {:02X} __ __ | EOR #${:02X}", opcode, next_byte, next_byte),
            },
            0x4D => format!("{:02X} {:02X} {:02X} __ | EOR ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x4F => format!("{:02X} {:02X} {:02X} {:02X} | EOR ${:06X}", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0x45 => format!("{:02X} {:02X} __ __ | EOR ${:02X} | dp", opcode, next_byte, next_byte),
            0x52 => format!("{:02X} {:02X} __ __ | EOR (${:02X})", opcode, next_byte, next_byte),
            0x47 => format!("{:02X} {:02X} __ __ | EOR [${:02X}]", opcode, next_byte, next_byte),
            0x5D => format!("{:02X} {:02X} {:02X} __ | EOR ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0x5F => format!("{:02X} {:02X} {:02X} {:02X} | EOR ${:06X}, X", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0x59 => format!("{:02X} {:02X} {:02X} __ | EOR ${:04X}, Y", opcode, next_byte, next_second_byte, next_word),
            0x55 => format!("{:02X} {:02X} __ __ | EOR ${:02X}, X | dp", opcode, next_byte, next_byte),
            0x41 => format!("{:02X} {:02X} __ __ | EOR (${:02X}, X) | dp", opcode, next_byte, next_byte),
            0x51 => format!("{:02X} {:02X} __ __ | EOR (${:02X}), Y | dp", opcode, next_byte, next_byte),
            0x57 => format!("{:02X} {:02X} __ __ | EOR [${:02X}], Y | dp", opcode, next_byte, next_byte),
            0x43 => format!("{:02X} {:02X} __ __ | EOR {:02X}, S | sr", opcode, next_byte, next_byte),
            0x53 => format!("{:02X} {:02X} __ __ | EOR ({:02X}, S), Y | sr", opcode, next_byte, next_byte),
            // INC
            0x1A => format!("{:02X} __ __ __ | INC A", opcode),
            0xEE => format!("{:02X} {:02X} {:02X} __ | INC ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0xE6 => format!("{:02X} {:02X} __ __ | INC ${:02X} | dp", opcode, next_byte, next_byte),
            0xFE => format!("{:02X} {:02X} {:02X} __ | INC ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0xF6 => format!("{:02X} {:02X} __ __ | INC ${:02X}, X | dp", opcode, next_byte, next_byte),
            // INX
            0xE8 => format!("{:02X} __ __ __ | INX", opcode),
            // INY
            0xC8 => format!("{:02X} __ __ __ | INY", opcode),
            // JMP
            0x4C => format!("{:02X} {:02X} {:02X} __ | JMP ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x6C => format!("{:02X} {:02X} {:02X} __ | JMP (${:04X})", opcode, next_byte, next_second_byte, next_word),
            0x7C => format!("{:02X} {:02X} {:02X} __ | JMP (${:04X}, X)", opcode, next_byte, next_second_byte, next_word),
            0x5C => format!("{:02X} {:02X} {:02X} {:02X} | JMP ${:06X}", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0xDC => format!("{:02X} {:02X} {:02X} __ | JMP [${:04X}]", opcode, next_byte, next_second_byte, next_word),
            // JSR
            0x20 => format!("{:02X} {:02X} {:02X} __ | JSR ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0xFC => format!("{:02X} {:02X} {:02X} __ | JSR (${:04X}, X)", opcode, next_byte, next_second_byte, next_word),
            0x22 => format!("{:02X} {:02X} {:02X} {:02X} | JSR ${:06X}", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            // LDA
            0xA9 => match is_cpu_16bit {
                true =>   format!("{:02X} {:02X} {:02X} __ | LDA #${:04X}", opcode, next_byte, next_second_byte, next_word),
                false =>  format!("{:02X} {:02X} __ __ | LDA #${:02X}", opcode, next_byte, next_byte),
            },
            0xAD => format!("{:02X} {:02X} {:02X} __ | LDA ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0xAF => format!("{:02X} {:02X} {:02X} {:02X} | LDA ${:06X}", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0xA5 => format!("{:02X} {:02X} __ __ | LDA ${:02X} | dp", opcode, next_byte, next_byte),
            0xB2 => format!("{:02X} {:02X} __ __ | LDA (${:02X})", opcode, next_byte, next_byte),
            0xA7 => format!("{:02X} {:02X} __ __ | LDA [${:02X}]", opcode, next_byte, next_byte),
            0xBD => format!("{:02X} {:02X} {:02X} __ | LDA ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0xBF => format!("{:02X} {:02X} {:02X} {:02X} | LDA ${:06X}, X", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0xB9 => format!("{:02X} {:02X} {:02X} __ | LDA ${:04X}, Y", opcode, next_byte, next_second_byte, next_word),
            0xB5 => format!("{:02X} {:02X} __ __ | LDA ${:02X}, X | dp", opcode, next_byte, next_byte),
            0xA1 => format!("{:02X} {:02X} __ __ | LDA (${:02X}, X) | dp", opcode, next_byte, next_byte),
            0xB1 => format!("{:02X} {:02X} __ __ | LDA (${:02X}), Y | dp", opcode, next_byte, next_byte),
            0xB7 => format!("{:02X} {:02X} __ __ | LDA [${:02X}], Y | dp", opcode, next_byte, next_byte),
            0xA3 => format!("{:02X} {:02X} __ __ | LDA {:02X}, S | sr", opcode, next_byte, next_byte),
            0xB3 => format!("{:02X} {:02X} __ __ | LDA ({:02X}, S), Y | sr", opcode, next_byte, next_byte),
            // LDX
            0xA2 => match is_index_16bit {
                true =>   format!("{:02X} {:02X} {:02X} __ | LDX #${:04X}", opcode, next_byte, next_second_byte, next_word),
                false =>  format!("{:02X} {:02X} __ __ | LDX #${:02X}", opcode, next_byte, next_byte),
            },
            0xAE => format!("{:02X} {:02X} {:02X} __ | LDX ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0xA6 => format!("{:02X} {:02X} __ __ | LDX ${:02X} | dp", opcode, next_byte, next_byte),
            0xBE => format!("{:02X} {:02X} {:02X} __ | LDX ${:04X}, Y", opcode, next_byte, next_second_byte, next_word),
            0xB6 => format!("{:02X} {:02X} __ __ | LDX ${:02X}, Y | dp", opcode, next_byte, next_byte),
            // LDY
            0xA0 => match is_index_16bit {
                true =>   format!("{:02X} {:02X} {:02X} __ | LDY #${:04X}", opcode, next_byte, next_second_byte, next_word),
                false =>  format!("{:02X} {:02X} __ __ | LDY #${:02X}", opcode, next_byte, next_byte),
            },
            0xAC => format!("{:02X} {:02X} {:02X} __ | LDY ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0xA4 => format!("{:02X} {:02X} __ __ | LDY ${:02X} | dp", opcode, next_byte, next_byte),
            0xB4 => format!("{:02X} {:02X} {:02X} __ | LDY ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0xBC => format!("{:02X} {:02X} __ __ | LDY ${:02X}, X | dp", opcode, next_byte, next_byte),
            // LSR
            0x4A => format!("{:02X} __ __ __ | LSR A", opcode),
            0x4E => format!("{:02X} {:02X} {:02X} __ | LSR ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x46 => format!("{:02X} {:02X} __ __ | LSR ${:02X} | dp", opcode, next_byte, next_byte),
            0x5E => format!("{:02X} {:02X} {:02X} __ | LSR ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0x56 => format!("{:02X} {:02X} __ __ | LSR ${:02X}, X | dp", opcode, next_byte, next_byte),
            // MVN
            0x54 => format!("{:02X} {:02X} {:02X} __ | MVN ${:02X},${:02X}", opcode, next_byte, next_second_byte, next_second_byte, next_byte),
            // MVP
            0x44 => format!("{:02X} {:02X} {:02X} __ | MVP ${:02X},${:02X}", opcode, next_byte, next_second_byte, next_second_byte, next_byte),
            // NOP
            0xEA => format!("{:02X} __ __ __ | NOP", opcode),
            // ORA
            0x09 => match is_cpu_16bit {
                true =>   format!("{:02X} {:02X} {:02X} __ | ORA #${:04X}", opcode, next_byte, next_second_byte, next_word),
                false =>  format!("{:02X} {:02X} __ __ | ORA #${:02X}", opcode, next_byte, next_byte),
            },
            0x0D => format!("{:02X} {:02X} {:02X} __ | ORA ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x0F => format!("{:02X} {:02X} {:02X} {:02X} | ORA ${:06X}", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0x05 => format!("{:02X} {:02X} __ __ | ORA ${:02X} | dp", opcode, next_byte, next_byte),
            0x12 => format!("{:02X} {:02X} __ __ | ORA (${:02X})", opcode, next_byte, next_byte),
            0x07 => format!("{:02X} {:02X} __ __ | ORA [${:02X}]", opcode, next_byte, next_byte),
            0x1D => format!("{:02X} {:02X} {:02X} __ | ORA ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0x1F => format!("{:02X} {:02X} {:02X} {:02X} | ORA ${:06X}, X", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0x19 => format!("{:02X} {:02X} {:02X} __ | ORA ${:04X}, Y", opcode, next_byte, next_second_byte, next_word),
            0x15 => format!("{:02X} {:02X} __ __ | ORA ${:02X}, X | dp", opcode, next_byte, next_byte),
            0x01 => format!("{:02X} {:02X} __ __ | ORA (${:02X}, X) | dp", opcode, next_byte, next_byte),
            0x11 => format!("{:02X} {:02X} __ __ | ORA (${:02X}), Y | dp", opcode, next_byte, next_byte),
            0x17 => format!("{:02X} {:02X} __ __ | ORA [${:02X}], Y | dp", opcode, next_byte, next_byte),
            0x03 => format!("{:02X} {:02X} __ __ | ORA {:02X}, S | sr", opcode, next_byte, next_byte),
            0x13 => format!("{:02X} {:02X} __ __ | ORA ({:02X}, S), Y | sr", opcode, next_byte, next_byte),
            // PEA
            0xF4 => format!("{:02X} {:02X} {:02X} __ | PEA ${:04X}", opcode, next_byte, next_second_byte, next_word),
            // PEI
            0xD4 => format!("{:02X} {:02X} __ __ | PEI (${:02X}) | dp", opcode, next_byte, next_byte),
            // PER
            0x62 => format!("{:02X} {:02X} {:02X} __ | PER ${:04X}", opcode, next_byte, next_second_byte, next_word),
            // PHA
            0x48 => format!("{:02X} __ __ __ | PHA", opcode),
            // PHB
            0x8B => format!("{:02X} __ __ __ | PHB", opcode),
            // PHD
            0x0B => format!("{:02X} __ __ __ | PHD", opcode),
            // PHK
            0x4B => format!("{:02X} __ __ __ | PHK", opcode),
            // PHP
            0x08 => format!("{:02X} __ __ __ | PHP", opcode),
            // PHX
            0xDA => format!("{:02X} __ __ __ | PHX", opcode),
            // PHY
            0x5A => format!("{:02X} __ __ __ | PHY", opcode),
            // PLA
            0x68 => format!("{:02X} __ __ __ | PLA", opcode),
            // PLB
            0xAB => format!("{:02X} __ __ __ | PLB", opcode),
            // PLD
            0x2B => format!("{:02X} __ __ __ | PLD", opcode),
            // PLP
            0x28 => format!("{:02X} __ __ __ | PLP", opcode),
            // PLX
            0xFA => format!("{:02X} __ __ __ | PLX", opcode),
            // PLY
            0x7A => format!("{:02X} __ __ __ | PLY", opcode),
            // REP
            0xC2 => format!("{:02X} {:02X} __ __ | REP #${:02X}", opcode, next_byte, next_byte),
            // ROL
            0x2A => format!("{:02X} __ __ __ | ROL A", opcode),
            0x2E => format!("{:02X} {:02X} {:02X} __ | ROL ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x26 => format!("{:02X} {:02X} __ __ | ROL ${:02X} | dp", opcode, next_byte, next_byte),
            0x3E => format!("{:02X} {:02X} {:02X} __ | ROL ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0x36 => format!("{:02X} {:02X} __ __ | ROL ${:02X}, X | dp", opcode, next_byte, next_byte),
            // ROR
            0x6A => format!("{:02X} __ __ __ | ROR A", opcode),
            0x6E => format!("{:02X} {:02X} {:02X} __ | ROR ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x66 => format!("{:02X} {:02X} __ __ | ROR ${:02X} | dp", opcode, next_byte, next_byte),
            0x7E => format!("{:02X} {:02X} {:02X} __ | ROR ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0x76 => format!("{:02X} {:02X} __ __ | ROR ${:02X}, X | dp", opcode, next_byte, next_byte),
            // RTI
            0x40 => format!("{:02X} __ __ __ | RTI", opcode),
            // RTL
            0x6B => format!("{:02X} __ __ __ | RTL", opcode),
            // RTS
            0x60 => format!("{:02X} __ __ __ | RTS", opcode),
            // SBC
            0xE9 => match is_cpu_16bit {
                true =>   format!("{:02X} {:02X} {:02X} __ | SBC #${:04X}", opcode, next_byte, next_second_byte, next_word),
                false =>  format!("{:02X} {:02X} __ __ | SBC #${:02X}", opcode, next_byte, next_byte),
            },
            0xED => format!("{:02X} {:02X} {:02X} __ | SBC ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0xEF => format!("{:02X} {:02X} {:02X} {:02X} | SBC ${:06X}", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0xE5 => format!("{:02X} {:02X} __ __ | SBC ${:02X} | dp", opcode, next_byte, next_byte),
            0xF2 => format!("{:02X} {:02X} __ __ | SBC (${:02X})", opcode, next_byte, next_byte),
            0xE7 => format!("{:02X} {:02X} __ __ | SBC [${:02X}]", opcode, next_byte, next_byte),
            0xFD => format!("{:02X} {:02X} {:02X} __ | SBC ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0xFF => format!("{:02X} {:02X} {:02X} {:02X} | SBC ${:06X}, X", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0xF9 => format!("{:02X} {:02X} {:02X} __ | SBC ${:04X}, Y", opcode, next_byte, next_second_byte, next_word),
            0xF5 => format!("{:02X} {:02X} __ __ | SBC ${:02X}, X | dp", opcode, next_byte, next_byte),
            0xE1 => format!("{:02X} {:02X} __ __ | SBC (${:02X}, X) | dp", opcode, next_byte, next_byte),
            0xF1 => format!("{:02X} {:02X} __ __ | SBC (${:02X}), Y | dp", opcode, next_byte, next_byte),
            0xF7 => format!("{:02X} {:02X} __ __ | SBC [${:02X}], Y | dp", opcode, next_byte, next_byte),
            0xE3 => format!("{:02X} {:02X} __ __ | SBC {:02X}, S | sr", opcode, next_byte, next_byte),
            0xF3 => format!("{:02X} {:02X} __ __ | SBC ({:02X}, S), Y | sr", opcode, next_byte, next_byte),
            // SEC
            0x38 => format!("{:02X} __ __ __ | SEC", opcode),
            // SED
            0xF8 => format!("{:02X} __ __ __ | SED", opcode),
            // SEI
            0x78 => format!("{:02X} __ __ __ | SEI", opcode),
            // SEP
            0xE2 => format!("{:02X} {:02X} __ __ | SEP #${:02X}", opcode, next_byte, next_byte),
            // STA
            0x8D => format!("{:02X} {:02X} {:02X} __ | STA ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x8F => format!("{:02X} {:02X} {:02X} {:02X} | STA ${:06X}", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0x85 => format!("{:02X} {:02X} __ __ | STA ${:02X} | dp", opcode, next_byte, next_byte),
            0x92 => format!("{:02X} {:02X} __ __ | STA (${:02X})", opcode, next_byte, next_byte),
            0x87 => format!("{:02X} {:02X} __ __ | STA [${:02X}]", opcode, next_byte, next_byte),
            0x9D => format!("{:02X} {:02X} {:02X} __ | STA ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0x9F => format!("{:02X} {:02X} {:02X} {:02X} | STA ${:06X}, X", opcode, next_byte, next_second_byte, next_third_byte, next_word_long),
            0x99 => format!("{:02X} {:02X} {:02X} __ | STA ${:04X}, Y", opcode, next_byte, next_second_byte, next_word),
            0x95 => format!("{:02X} {:02X} __ __ | STA ${:02X}, X | dp", opcode, next_byte, next_byte),
            0x81 => format!("{:02X} {:02X} __ __ | STA (${:02X}, X) | dp", opcode, next_byte, next_byte),
            0x91 => format!("{:02X} {:02X} __ __ | STA (${:02X}), Y | dp", opcode, next_byte, next_byte),
            0x97 => format!("{:02X} {:02X} __ __ | STA [${:02X}], Y | dp", opcode, next_byte, next_byte),
            0x83 => format!("{:02X} {:02X} __ __ | STA {:02X}, S | sr", opcode, next_byte, next_byte),
            0x93 => format!("{:02X} {:02X} __ __ | STA ({:02X}, S), Y | sr", opcode, next_byte, next_byte),
            // STP
            0xDB => format!("{:02X} __ __ __ | STP", opcode),
            // STX
            0x8E => format!("{:02X} {:02X} {:02X} __ | STX ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x86 => format!("{:02X} {:02X} __ __ | STX ${:02X} | dp", opcode, next_byte, next_byte),
            0x96 => format!("{:02X} {:02X} __ __ | STX ${:02X}, Y | dp", opcode, next_byte, next_byte),
            // STY
            0x8C => format!("{:02X} {:02X} {:02X} __ | STY ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x84 => format!("{:02X} {:02X} __ __ | STY ${:02X} | dp", opcode, next_byte, next_byte),
            0x94 => format!("{:02X} {:02X} __ __ | STY ${:02X}, X | dp", opcode, next_byte, next_byte),
            // STZ
            0x9C => format!("{:02X} {:02X} {:02X} __ | STZ ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x64 => format!("{:02X} {:02X} __ __ | STZ ${:02X} | dp", opcode, next_byte, next_byte),
            0x9E => format!("{:02X} {:02X} {:02X} __ | STZ ${:04X}, X", opcode, next_byte, next_second_byte, next_word),
            0x74 => format!("{:02X} {:02X} __ __ | STZ ${:02X}, X | dp", opcode, next_byte, next_byte),
            // TAX
            0xAA => format!("{:02X} __ __ __ | TAX", opcode),
            // TAY
            0xA8 => format!("{:02X} __ __ __ | TAY", opcode),
            // TCD
            0x5B => format!("{:02X} __ __ __ | TCD", opcode),
            // TCS
            0x1B => format!("{:02X} __ __ __ | TCS", opcode),
            // TCD
            0x7B => format!("{:02X} __ __ __ | TDC", opcode),
            // TRB
            0x1C => format!("{:02X} {:02X} {:02X} __ | TRB ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x14 => format!("{:02X} {:02X} __ __ | TRB ${:02X} | dp", opcode, next_byte, next_byte),
            // TSB
            0x0C => format!("{:02X} {:02X} {:02X} __ | TSB ${:04X}", opcode, next_byte, next_second_byte, next_word),
            0x04 => format!("{:02X} {:02X} __ __ | TSB ${:02X} | dp", opcode, next_byte, next_byte),
            // TSC
            0x3B => format!("{:02X} __ __ __ | TSC", opcode),
            // TSX
            0xBA => format!("{:02X} __ __ __ | TSX", opcode),
            // TXA
            0x8A => format!("{:02X} __ __ __ | TXA", opcode),
            // TXS
            0x9A => format!("{:02X} __ __ __ | TXS", opcode),
            // TXY
            0x9B => format!("{:02X} __ __ __ | TXY", opcode),
            // TYA
            0x98 => format!("{:02X} __ __ __ | TYA", opcode),
            // TYX
            0xBB => format!("{:02X} __ __ __ | TYX", opcode),
            // WAI
            0xCB => format!("{:02X} __ __ __ | WAI", opcode),
            // WDM
            0x42 => format!("{:02X} __ __ __ | WDM", opcode),
            // XBA
            0xEB => format!("{:02X} __ __ __ | XBA", opcode),
            // XCE
            0xFB => format!("{:02X} __ __ __ | XCE", opcode),
        }
    }
}
