mod bit_helpers;
mod cpu;
mod utils;

use cpu::cpu::Cpu;
use cpu::memory::Memory;
use std::collections::HashMap;

fn main() {
    let mut memory = Memory::new();
    // Add
    memory.set_byte(0, 0x80);
    memory.set_byte(1, 0x81);
    memory.set_byte(2, 0x82);
    memory.set_byte(3, 0x83);
    memory.set_byte(4, 0x84);
    memory.set_byte(5, 0x85);
    memory.set_byte(6, 0x87);
    // Subtract
    memory.set_byte(7, 0x90);
    memory.set_byte(8, 0x91);
    memory.set_byte(9, 0x92);
    memory.set_byte(10, 0x93);
    memory.set_byte(11, 0x94);
    memory.set_byte(12, 0x95);
    memory.set_byte(13, 0x97);
    // XOR
    memory.set_byte(14, 0xA8);
    memory.set_byte(15, 0xA9);
    memory.set_byte(16, 0xAA);
    memory.set_byte(17, 0xAB);
    memory.set_byte(18, 0xAC);
    memory.set_byte(19, 0xAD);
    memory.set_byte(20, 0xAF);
    // LOAD
    memory.set_byte(21, 0x78);
    memory.set_byte(22, 0x79);
    memory.set_byte(23, 0x7A);
    memory.set_byte(24, 0x7B);
    memory.set_byte(25, 0x7C);
    memory.set_byte(26, 0x7D);
    memory.set_byte(27, 0x7F);
    
	let mut cpu = Cpu::new(memory);
    cpu._registers.borrow_mut().b.set(1);
    cpu._registers.borrow_mut().c.set(2);
    cpu._registers.borrow_mut().d.set(4);
    cpu._registers.borrow_mut().e.set(8);
    cpu._registers.borrow_mut().h.set(16);
    cpu._registers.borrow_mut().l.set(32);
    
    for x in 0..7 {    
        cpu.run_next_instruction();
        println!("{}", as_hex!(cpu._registers.borrow_mut().a));
    }
    
    cpu._registers.borrow_mut().a.set(0xFF);
    
    for x in 0..7 {    
        cpu.run_next_instruction();
        println!("{}", as_hex!(cpu._registers.borrow_mut().a));
    }
    
    cpu._registers.borrow_mut().a.set(0xFF);
    
    for x in 0..7 {    
        cpu.run_next_instruction();
        println!("{}", as_hex!(cpu._registers.borrow_mut().a));
    }
    
    cpu._registers.borrow_mut().a.set(0x00);
    
    for x in 0..7 {    
        cpu.run_next_instruction();
        println!("{}", as_hex!(cpu._registers.borrow_mut().a));
    }
    
    /*
    println!("Hello, world!");
    
    let mut book_reviews = HashMap::new();
    book_reviews.insert(
        "Mistborn",
        "Totally awesome!"
    );
    
    println!("{}", book_reviews["Mistborn"]);*/
}
