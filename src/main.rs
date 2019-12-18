pub mod bit_helpers;
pub mod registers;

use registers::Registers;

fn main() {
    let registers = Registers {a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0};
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    pub mod test_utils;
}