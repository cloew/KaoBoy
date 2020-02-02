mod utils;
mod bit_helpers;
mod cpu;
mod instructions;
mod registers;

use registers::registers::Registers;
use cpu::cpu::Cpu;
use cpu::memory::Memory;
use std::collections::HashMap;

fn main() {
    let mut memory = Memory::new();
    memory.set_byte(0, 0x80);
    memory.set_byte(1, 0x81);
    memory.set_byte(2, 0x82);
    memory.set_byte(3, 0x83);
    memory.set_byte(4, 0x84);
    memory.set_byte(5, 0x85);
    memory.set_byte(6, 0x87);
    
	let mut cpu = Cpu::new(memory);
    cpu._registers.b.set(1);
    cpu._registers.c.set(2);
    cpu._registers.d.set(4);
    cpu._registers.e.set(8);
    cpu._registers.h.set(16);
    cpu._registers.l.set(32);
    
    for x in 0..7 {    
        cpu.run_next_instruction();
        println!("{}", as_hex!(cpu._registers.a));
    }
    
    /*let mut registers = Registers::new();
    println!("Hello, world!");
    
    let mut book_reviews = HashMap::new();
    book_reviews.insert(
        "Mistborn",
        "Totally awesome!"
    );
    
    println!("{}", book_reviews["Mistborn"]);*/
}
