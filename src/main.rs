mod bit_helpers;
mod cpu;
mod emulator;
mod utils;

use emulator::Emulator;
use cpu::registers::{DoubleRegisterName, RegisterName};

fn main() {
    let mut emulator = Emulator::new();
    
    let program: [u8; 81] = [
        // Add
        0x80, // 0x00+0x01 = 0x01
        0x81, // 0x01+0x02 = 0x03
        0x82, // 0x03+0x04 = 0x07
        0x83, // 0x07+0x08 = 0x0F
        0x84, // 0x0F+0x10 = 0x1F
        0x85, // 0x1F+0x20 = 0x3F
        0x87, // 0x3F+0x3F = 0x7E
        0xC6, // 0x7E+0x01 = 0x7F
        0x01,
        // Subtract
        0x90,
        0x91,
        0x92,
        0x93,
        0x94,
        0x95,
        0x97,
        0xD6,
        0x01,
        // XOR
        0xA8,
        0xA9,
        0xAA,
        0xAB,
        0xAC,
        0xAD,
        0xAF,
        0xEE,
        0xF0,
        0xEE,
        0x0F,
        // LOAD A
        0x78,
        0x79,
        0x7A,
        0x7B,
        0x7C,
        0x7D,
        0x7F,
        0x3E,
        0xFF,
        // Load Short Registers
        0x01,
        0x12,
        0x34,
        0x11,
        0x56,
        0x78,
        0x21,
        0xFE,
        0xDC,
        0x31,
        0xCA,
        0xB0,
        // LOAD (HL)
        0x36,
        0x99,
        0x70,
        0x71,
        0x72,
        0x73,
        0x74,
        0x75,
        0x77,
        // LOAD (BC)
        0x02,
        0xCA,
        // LOAD (DE)
        0x12,
        0xBE,
        // LOAD (HL+)
        0x22,
        // LOAD (HL-)
        0x32,
        // INC Registers
        0x04,
        0x14,
        0x24,
        0x0C,
        0x1C,
        0x2C,
        0x3C,
        // INC Double Registers
        0x03,
        0x13,
        0x23,
        //BIT Instruction test
        0xCB,
        0x40,
        //JR Instruction test
        0x20,
        0x40,
        0x28,
        0x40,
    ];
    
    
    emulator.bootstrap(&program);
    
    emulator._cpu._registers.borrow_mut().b.set(1);
    emulator._cpu._registers.borrow_mut().c.set(2);
    emulator._cpu._registers.borrow_mut().d.set(4);
    emulator._cpu._registers.borrow_mut().e.set(8);
    emulator._cpu._registers.borrow_mut().h.set(16);
    emulator._cpu._registers.borrow_mut().l.set(32);
    
    println!("Testing add");
    for _x in 0..8 {    
        emulator._cpu.run_next_instruction();
        println!("{}", as_hex!(emulator._cpu._registers.borrow_mut().a));
    }
    println!("");
    
    println!("Testing subtract");
    emulator._cpu._registers.borrow_mut().a.set(0xFF);
    for _x in 0..8 {    
        emulator._cpu.run_next_instruction();
        println!("{}", as_hex!(emulator._cpu._registers.borrow_mut().a));
    }
    println!("");
    
    println!("Testing xor");
    emulator._cpu._registers.borrow_mut().a.set(0xFF);
    for _x in 0..9 {    
        emulator._cpu.run_next_instruction();
        println!("{}", as_hex!(emulator._cpu._registers.borrow_mut().a));
    }
    println!("");
    
    println!("Testing load A");
    emulator._cpu._registers.borrow_mut().a.set(0x00);
    for _x in 0..8 {    
        emulator._cpu.run_next_instruction();
        println!("{}", as_hex!(emulator._cpu._registers.borrow_mut().a));
    }
    
    println!("Testing load BC");
    emulator._cpu.run_next_instruction();
    println!("BC: {}", as_hex!(emulator._cpu._registers.borrow_mut().bc));
    println!("Testing load DE");
    emulator._cpu.run_next_instruction();
    println!("DE: {}", as_hex!(emulator._cpu._registers.borrow_mut().de));
    println!("Testing load HL");
    emulator._cpu.run_next_instruction();
    println!("HL: {}", as_hex!(emulator._cpu._registers.borrow_mut().hl));
    println!("Testing load Stack pointer");
    emulator._cpu.run_next_instruction();
    println!("Stack Pointer: {}", as_hex!(emulator._cpu._stack.borrow_mut().get_pointer()));
    
    println!("Testing load (HL)");
    emulator._cpu._registers.borrow_mut().a.set(0x00);
    for _x in 0..8 {
        let address = emulator._cpu._registers.borrow_mut().hl.get();
        emulator._cpu.run_next_instruction();
        println!("{}", as_hex!(emulator._memory.borrow().read_byte(address)));
    }
    println!("Testing load (BC)");
    let address = emulator._cpu._registers.borrow_mut().bc.get();
    emulator._cpu.run_next_instruction();
    println!("BC: {}", as_hex!(emulator._memory.borrow().read_byte(address)));
    println!("Testing load (DE)");
    let address = emulator._cpu._registers.borrow_mut().de.get();
    emulator._cpu.run_next_instruction();
    println!("DE: {}", as_hex!(emulator._memory.borrow().read_byte(address)));
    
    println!("Testing load (HL+)");
    emulator._cpu._registers.borrow_mut().a.set(0x12);
    emulator._cpu._registers.borrow_mut().hl.set(0x1234);
    let address = emulator._cpu._registers.borrow_mut().hl.get();
    emulator._cpu.run_next_instruction();
    println!("(HL): {}", as_hex!(emulator._memory.borrow().read_byte(address)));
    println!("HL: {}", as_hex!(emulator._cpu._registers.borrow_mut().hl));
    
    println!("Testing load (HL-)");
    emulator._cpu._registers.borrow_mut().a.set(0x12);
    emulator._cpu._registers.borrow_mut().hl.set(0x1234);
    let address = emulator._cpu._registers.borrow_mut().hl.get();
    emulator._cpu.run_next_instruction();
    println!("(HL): {}", as_hex!(emulator._memory.borrow().read_byte(address)));
    println!("HL: {}", as_hex!(emulator._cpu._registers.borrow_mut().hl));
    
    println!("Testing INC Registers");
    emulator._cpu._registers.borrow_mut().a.set(0x00);
    emulator._cpu._registers.borrow_mut().b.set(0x1);
    emulator._cpu._registers.borrow_mut().c.set(0x2);
    emulator._cpu._registers.borrow_mut().d.set(0x4);
    emulator._cpu._registers.borrow_mut().e.set(0x8);
    emulator._cpu._registers.borrow_mut().h.set(0x10);
    emulator._cpu._registers.borrow_mut().l.set(0x20);
    let registers = [
        RegisterName::B,
        RegisterName::D,
        RegisterName::H,
        RegisterName::C,
        RegisterName::E,
        RegisterName::L,
        RegisterName::A,
    ];
    for name in registers.iter() {
        emulator._cpu.run_next_instruction();
        println!("{}", as_hex!(emulator._cpu._registers.borrow_mut().get(*name)));
    }
    
    println!("Testing INC Double Registers");
    emulator._cpu._registers.borrow_mut().a.set(0x00);
    emulator._cpu._registers.borrow_mut().b.set(0x1);
    emulator._cpu._registers.borrow_mut().c.set(0x2);
    emulator._cpu._registers.borrow_mut().d.set(0x4);
    emulator._cpu._registers.borrow_mut().e.set(0x8);
    emulator._cpu._registers.borrow_mut().h.set(0x10);
    emulator._cpu._registers.borrow_mut().l.set(0x20);
    let registers = [
        DoubleRegisterName::BC,
        DoubleRegisterName::DE,
        DoubleRegisterName::HL,
    ];
    for name in registers.iter() {
        emulator._cpu.run_next_instruction();
        println!("{}", as_hex!(emulator._cpu._registers.borrow_mut().get_double(*name)));
    }
    
    println!("Testing load BIT");
    emulator._cpu._registers.borrow_mut().b.set(0x00);
    emulator._cpu._registers.borrow_mut().zero_flag.reset();
    emulator._cpu._registers.borrow_mut().subtract_flag.activate();
    emulator._cpu._registers.borrow_mut().half_carry_flag.reset();
    emulator._cpu._registers.borrow_mut().carry_flag.activate();
    emulator._cpu.run_next_instruction();
    println!("Zero Flag: {}", emulator._cpu._registers.borrow().zero_flag.get());
    println!("Subtract Flag: {}", emulator._cpu._registers.borrow().subtract_flag.get());
    println!("Carry Flag: {}", emulator._cpu._registers.borrow().carry_flag.get());
    println!("Half Carry Flag: {}", emulator._cpu._registers.borrow().half_carry_flag.get());
    
    println!("Testing load JR");
    emulator._cpu._registers.borrow_mut().zero_flag.activate();
    println!("Program Counter Before: {}", emulator._cpu._counter.borrow().get_counter());
    emulator._cpu.run_next_instruction();
    println!("Program Counter After JR NZ: {}", emulator._cpu._counter.borrow().get_counter());
    emulator._cpu.run_next_instruction();
    println!("Program Counter After JR Z: {}", emulator._cpu._counter.borrow().get_counter());
}
