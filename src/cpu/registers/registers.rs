use super::register::Register;
use super::register_flag::RegisterFlag;
use super::register_names::RegisterName;
use crate::{build_u16, get_lower_u8, get_upper_u8};

use std::rc::Rc;
use std::cell::RefCell;

const CARRY_FLAG_MASK: u8 = 0x10;
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
    pub carry_flag: RegisterFlag,
    
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
            carry_flag: RegisterFlag::new(registers_ref.clone(), RegisterName::F, CARRY_FLAG_MASK),
            
            // Underlying register array
            _registers: registers_ref};
	}
    
    pub fn get(&self, register_name: RegisterName) -> &Register {
        return match register_name {
            RegisterName::A => &self.a,
            RegisterName::B => &self.b,
            RegisterName::C => &self.c,
            RegisterName::D => &self.d,
            RegisterName::E => &self.e,
            RegisterName::F => &self.f,
            RegisterName::H => &self.h,
            RegisterName::L => &self.l,
        }
    }
    
    pub fn get_mut(&mut self, register_name: RegisterName) -> &mut Register {
        return match register_name {
            RegisterName::A => &mut self.a,
            RegisterName::B => &mut self.b,
            RegisterName::C => &mut self.c,
            RegisterName::D => &mut self.d,
            RegisterName::E => &mut self.e,
            RegisterName::F => &mut self.f,
            RegisterName::H => &mut self.h,
            RegisterName::L => &mut self.l,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;
    
    #[test]
    fn test_get_gets_proper_register() {
        let registers = Registers::new();
        
        let register = registers.get(RegisterName::A);
        
        assert!(ptr::eq(register, &registers.a));
    }
}