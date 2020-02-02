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
    memory.set_byte(0, 0x87);
    
	let mut cpu = Cpu::new(memory);
    cpu._registers.a.set(1);
    cpu.run_next_instruction();
    
    println!("{}", as_hex!(cpu._registers.a));
    
    /*let mut registers = Registers::new();
    println!("Hello, world!");
    
    let mut book_reviews = HashMap::new();
    book_reviews.insert(
        "Mistborn",
        "Totally awesome!"
    );
    
    println!("{}", book_reviews["Mistborn"]);*/
}
