use crate::{build_u16, get_lower_u8, get_upper_u8};
use crate::registers::register::{Register};
use crate::registers::register_flag::{RegisterFlag};
use crate::registers::register_names::{RegisterName};

use std::rc::Rc;
use std::cell::RefCell;

const HALF_CARRY_FLAG_MASK: u8 = 0x20;
const SUBTRACT_FLAG_MASK: u8 = 0x40;
const ZERO_FLAG_MASK: u8 = 0x80;

pub struct Registers {
    // Registers
    pub a: Register,
    pub b: Register,
    pub c: Register,
    pub d: Register,
    pub e: Register,
    pub f: Register, // Special Flags register
    pub h: Register,
    pub l: Register,
    
    // Flags
    pub zero_flag: RegisterFlag,
    pub subtract_flag: RegisterFlag,
    pub half_carry_flag: RegisterFlag,
    
    // Underlying register array
    pub _registers: Rc<RefCell<[u8; 8]>>,
}

impl Registers {
	pub fn new() -> Registers {
        let registers_ref = Rc::new(RefCell::new([0; 8]));
		return Registers {
            // Registers
            a: Register::new(registers_ref.clone(), RegisterName::A),
            b: Register::new(registers_ref.clone(), RegisterName::B),
            c: Register::new(registers_ref.clone(), RegisterName::C),
            d: Register::new(registers_ref.clone(), RegisterName::D),
            e: Register::new(registers_ref.clone(), RegisterName::E),
            f: Register::new(registers_ref.clone(), RegisterName::F),
            h: Register::new(registers_ref.clone(), RegisterName::H),
            l: Register::new(registers_ref.clone(), RegisterName::L),
            
            // Flags
            zero_flag: RegisterFlag::new(registers_ref.clone(), RegisterName::F, ZERO_FLAG_MASK),
            subtract_flag: RegisterFlag::new(registers_ref.clone(), RegisterName::F, SUBTRACT_FLAG_MASK),
            half_carry_flag: RegisterFlag::new(registers_ref.clone(), RegisterName::F, HALF_CARRY_FLAG_MASK),
            
            // Underlying register array
            _registers: registers_ref};
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
}