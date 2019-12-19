pub mod bit_helpers;
pub mod registers;
mod instructions;

use registers::Registers;
use std::collections::HashMap;

fn main() {
    let registers = Registers {a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0};
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