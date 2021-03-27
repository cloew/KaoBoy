pub mod bit_helpers;
pub mod registers;
mod instructions;

use registers::registers::Registers;

use std::rc::Rc;
use std::cell::RefCell;

struct Capsule {
  value: u8,
}

struct CompA {
	value: Rc<RefCell<Capsule>>,
}

impl CompA {
	
	pub fn increment(&mut self) {
		self.value.borrow_mut().value += 1;
	}
}

struct CompB {
	value: Rc<RefCell<Capsule>>,
}

impl CompB {
	
	pub fn decrement(&mut self) {
		self.value.borrow_mut().value -= 1;
	}
}

struct Thing {
	_value: Rc<RefCell<Capsule>>,
	
	comp_a: CompA,
	comp_b: CompB,
}

fn main() {
	let mut registers = Registers::new();
    println!("Hello, world!");
	
	let capsule = Capsule {value: 0};
	let mut value = Rc::new(RefCell::new(capsule));
	let mut thing = Thing {_value: value.clone(), comp_a: CompA {value: value.clone()}, comp_b: CompB {value: value.clone()}};
	
	println!("{}", thing._value.borrow().value);
	thing.comp_a.increment();
	println!("{}", thing._value.borrow().value);
	thing.comp_a.increment();
	println!("{}", thing._value.borrow().value);
	thing.comp_b.decrement();
	println!("{}", thing._value.borrow().value);
}

#[cfg(test)]
mod tests {
    pub mod test_utils;
}