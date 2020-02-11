mod bit_helpers;
mod emulator;
mod cpu;
mod utils;

use cpu::cpu::Cpu;
use emulator::Memory;

fn main() {
    let mut memory = Memory::new();
    
    let program: [u8; 38] = [
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
        // LOAD
        0x78,
        0x79,
        0x7A,
        0x7B,
        0x7C,
        0x7D,
        0x7F,
        0x3E,
        0xFF,
    ];
    
    
    memory.bootstrap(&program);
	let mut cpu = Cpu::new(rc_refcell!(memory));
    cpu._registers.borrow_mut().b.set(1);
    cpu._registers.borrow_mut().c.set(2);
    cpu._registers.borrow_mut().d.set(4);
    cpu._registers.borrow_mut().e.set(8);
    cpu._registers.borrow_mut().h.set(16);
    cpu._registers.borrow_mut().l.set(32);
    
    println!("Testing add");
    for _x in 0..8 {    
        cpu.run_next_instruction();
        println!("{}", as_hex!(cpu._registers.borrow_mut().a));
    }
    println!("");
    
    println!("Testing subtract");
    cpu._registers.borrow_mut().a.set(0xFF);
    for _x in 0..8 {    
        cpu.run_next_instruction();
        println!("{}", as_hex!(cpu._registers.borrow_mut().a));
    }
    println!("");
    
    println!("Testing xor");
    cpu._registers.borrow_mut().a.set(0xFF);
    for _x in 0..9 {    
        cpu.run_next_instruction();
        println!("{}", as_hex!(cpu._registers.borrow_mut().a));
    }
    println!("");
    
    println!("Testing load A");
    cpu._registers.borrow_mut().a.set(0x00);
    for _x in 0..8 {    
        cpu.run_next_instruction();
        println!("{}", as_hex!(cpu._registers.borrow_mut().a));
    }
}
