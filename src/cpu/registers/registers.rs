use super::register::Register;
use super::double_register::DoubleRegister;
use super::register_flag::RegisterFlag;
use super::register_names::RegisterName;
use super::double_register_names::DoubleRegisterName;
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
    
    // Double Registers
    pub af: DoubleRegister,
    pub bc: DoubleRegister,
    pub de: DoubleRegister,
    pub hl: DoubleRegister,
    
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
            
            // Double Registers
            af: DoubleRegister::new(registers_ref.clone(), RegisterName::A, RegisterName::F),
            bc: DoubleRegister::new(registers_ref.clone(), RegisterName::B, RegisterName::C),
            de: DoubleRegister::new(registers_ref.clone(), RegisterName::D, RegisterName::E),
            hl: DoubleRegister::new(registers_ref.clone(), RegisterName::H, RegisterName::L),
            
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
    
    pub fn get_double(&self, register_name: DoubleRegisterName) -> &DoubleRegister {
        return match register_name {
            DoubleRegisterName::AF => &self.af,
            DoubleRegisterName::BC => &self.bc,
            DoubleRegisterName::DE => &self.de,
            DoubleRegisterName::HL => &self.hl,
        }
    }
    
    pub fn get_double_mut(&mut self, register_name: DoubleRegisterName) -> &mut DoubleRegister {
        return match register_name {
            DoubleRegisterName::AF => &mut self.af,
            DoubleRegisterName::BC => &mut self.bc,
            DoubleRegisterName::DE => &mut self.de,
            DoubleRegisterName::HL => &mut self.hl,
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
    
    #[test]
    fn test_get_double_gets_proper_register() {
        let registers = Registers::new();
        
        let register = registers.get_double(DoubleRegisterName::HL);
        
        assert!(ptr::eq(register, &registers.hl));
    }
}