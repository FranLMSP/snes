use snes_core::cpu::bus::Bus;
use snes_core::cpu::registers::Registers;
/// https://github.com/TomHarte/ProcessorTests/tree/main/65816

use snes_core::emulator::Emulator;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use snes_core::rom::special_ram_cart::SpecialRAMCart;

use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct TestState {
    pc: usize,
    s: usize,
    p: usize,
    a: usize,
    x: usize,
    y: usize,
    dbr: usize,
    d: usize,
    pbr: usize,
    e: usize,
    ram: Vec<(usize, usize)>,
}

#[derive(Serialize, Deserialize)]
pub struct TestSuite {
    name: String,
    initial: TestState,
    r#final: TestState,
    cycles: Vec<(usize, Option<usize>, String)>,
}

#[derive(Serialize, Deserialize)]
pub struct TestSuiteList(Vec<TestSuite>);

fn print_failed(expected_state: &TestState, bus: &Bus, cpu_registers: &Registers) {
    eprintln!("FAILED!");
    eprintln!("----------");
    eprintln!("Expected:");
    eprintln!("{:?}", expected_state);
    eprintln!("Result:");
    let mut result_ram = vec![];
    for (address, _) in &expected_state.ram {
        result_ram.push((*address, bus.read_external(*address as u32) as usize))
    }
    let result_state = TestState {
        pc: cpu_registers.pc as usize,
        s: cpu_registers.sp as usize,
        p: cpu_registers.p as usize,
        a: cpu_registers.a as usize,
        x: cpu_registers.x as usize,
        y: cpu_registers.y as usize,
        dbr: cpu_registers.dbr as usize,
        d: cpu_registers.d as usize,
        pbr: cpu_registers.pbr as usize,
        e: if cpu_registers.emulation_mode {1} else {0},
        ram: result_ram,
    };
    eprintln!("{:?}", result_state);
    eprintln!("----------");
    std::process::exit(1);
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("A test file must be provided");
        std::process::exit(1);
    }
    let filename = args.get(1).unwrap();
    if filename.is_empty() {
        eprintln!("A test file must be provided");
        std::process::exit(1);
    }
    let mut file = File::open(filename).unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let tests: TestSuiteList = serde_json::from_str(&buff)?;

    let mut emulator = Emulator::new();
    emulator.bus.rom = Box::new(SpecialRAMCart::new());

    for test in tests.0 {
        println!("running test case {}", test.name);

        emulator.cpu.registers.pc = test.initial.pc as u16;
        emulator.cpu.registers.sp = test.initial.s as u16;
        emulator.cpu.registers.p = test.initial.p as u8;
        emulator.cpu.registers.a = test.initial.p as u16;
        emulator.cpu.registers.x = test.initial.x as u16;
        emulator.cpu.registers.y = test.initial.y as u16;
        emulator.cpu.registers.dbr = test.initial.dbr as u8;
        emulator.cpu.registers.d = test.initial.d as u16;
        emulator.cpu.registers.pbr = test.initial.pbr as u8;
        emulator.cpu.registers.emulation_mode = test.initial.e == 1;

        for (address, value) in test.initial.ram {
            emulator.bus.write(address as u32, value as u8);
        }

        emulator.tick();

        // compare the results
        if
            emulator.cpu.registers.pc as usize != test.r#final.pc ||
            emulator.cpu.registers.sp as usize != test.r#final.s ||
            emulator.cpu.registers.p as usize != test.r#final.p ||
            emulator.cpu.registers.a as usize != test.r#final.p ||
            emulator.cpu.registers.x as usize != test.r#final.x ||
            emulator.cpu.registers.y as usize != test.r#final.y ||
            emulator.cpu.registers.dbr as usize != test.r#final.dbr ||
            emulator.cpu.registers.d as usize != test.r#final.d ||
            emulator.cpu.registers.pbr as usize != test.r#final.pbr ||
            emulator.cpu.registers.emulation_mode != (test.r#final.e == 1)
        {
            print_failed(&test.r#final, &emulator.bus, &emulator.cpu.registers);
        }
        for (address, value) in &test.r#final.ram {
            if *value != emulator.bus.read_external(*address as u32) as usize {
                print_failed(&test.r#final, &emulator.bus, &emulator.cpu.registers);
            }
        }
    }

    println!("PASSED!");
    Ok(())
}
