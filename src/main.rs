pub mod bit_helpers;
pub mod registers;
mod instructions;

use registers::registers::Registers;
use std::collections::HashMap;

fn main() {
	let mut registers = Registers::new();
    println!("Hello, world!");
    
    let mut book_reviews = HashMap::new();
    book_reviews.insert(
        "Mistborn",
        "Totally awesome!"
    );
    
    println!("{}", book_reviews["Mistborn"]);
}

#[cfg(test)]
mod tests {
    pub mod test_utils;
}