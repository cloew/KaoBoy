mod utils;
mod bit_helpers;
mod cpu;
mod instructions;
mod registers;

use registers::registers::Registers;
use cpu::cpu::Cpu;
use std::collections::HashMap;

fn main() {
	let mut cpu = Cpu::new();
    
    /*let mut registers = Registers::new();
    println!("Hello, world!");
    
    let mut book_reviews = HashMap::new();
    book_reviews.insert(
        "Mistborn",
        "Totally awesome!"
    );
    
    println!("{}", book_reviews["Mistborn"]);*/
}
