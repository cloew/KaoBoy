mod bit_helpers;
mod cpu;
mod utils;

use cpu::cpu::Cpu;
use cpu::memory::Memory;
use std::collections::HashMap;

fn main() {
    let mut memory = Memory::new();
    // Add
    memory.set_byte(0, 0x80); // 0x00+0x01 = 0x01
    memory.set_byte(1, 0x81); // 0x01+0x02 = 0x03
    memory.set_byte(2, 0x82); // 0x03+0x04 = 0x07
    memory.set_byte(3, 0x83); // 0x07+0x08 = 0x0F
    memory.set_byte(4, 0x84); // 0x0F+0x10 = 0x1F
    memory.set_byte(5, 0x85); // 0x1F+0x20 = 0x3F
    memory.set_byte(6, 0x87); // 0x3F+0x3F = 0x7E
    memory.set_byte(7, 0xC6); // 0x7E+0x01 = 0x7F
    memory.set_byte(8, 0x01);
    // Subtract
    memory.set_byte(9, 0x90);
    memory.set_byte(10, 0x91);
    memory.set_byte(11, 0x92);
    memory.set_byte(12, 0x93);
    memory.set_byte(13, 0x94);
    memory.set_byte(14, 0x95);
    memory.set_byte(15, 0x97);
    memory.set_byte(16, 0xD6);
    memory.set_byte(17, 0x01);
    // XOR
    memory.set_byte(18, 0xA8);
    memory.set_byte(19, 0xA9);
    memory.set_byte(20, 0xAA);
    memory.set_byte(21, 0xAB);
    memory.set_byte(22, 0xAC);
    memory.set_byte(23, 0xAD);
    memory.set_byte(24, 0xAF);
    memory.set_byte(25, 0xEE);
    memory.set_byte(26, 0xF0);
    memory.set_byte(27, 0xEE);
    memory.set_byte(28, 0x0F);
    // LOAD
    memory.set_byte(29, 0x78);
    memory.set_byte(30, 0x79);
    memory.set_byte(31, 0x7A);
    memory.set_byte(32, 0x7B);
    memory.set_byte(33, 0x7C);
    memory.set_byte(34, 0x7D);
    memory.set_byte(35, 0x7F);
    memory.set_byte(36, 0x3E);
    memory.set_byte(37, 0xFF);
    
	let mut cpu = Cpu::new(memory);
    cpu._registers.borrow_mut().b.set(1);
    cpu._registers.borrow_mut().c.set(2);
    cpu._registers.borrow_mut().d.set(4);
    cpu._registers.borrow_mut().e.set(8);
    cpu._registers.borrow_mut().h.set(16);
    cpu._registers.borrow_mut().l.set(32);
    
    println!("Testing add");
    for x in 0..8 {    
        cpu.run_next_instruction();
        println!("{}", as_hex!(cpu._registers.borrow_mut().a));
    }
    println!("");
    
    println!("Testing subtract");
    cpu._registers.borrow_mut().a.set(0xFF);
    for x in 0..8 {    
        cpu.run_next_instruction();
        println!("{}", as_hex!(cpu._registers.borrow_mut().a));
    }
    println!("");
    
    println!("Testing xor");
    cpu._registers.borrow_mut().a.set(0xFF);
    for x in 0..9 {    
        cpu.run_next_instruction();
        println!("{}", as_hex!(cpu._registers.borrow_mut().a));
    }
    println!("");
    
    println!("Testing load A");
    cpu._registers.borrow_mut().a.set(0x00);
    for x in 0..8 {    
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
