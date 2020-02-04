use super::register_names::RegisterName;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Register {
	_registers: Rc<RefCell<[u8; 8]>>,
    _name: RegisterName,
}

impl Register {
	pub fn new(registers: Rc<RefCell<[u8; 8]>>, register_name: RegisterName) -> Register {
		return Register {_registers: registers, _name: register_name};
	}
	
	pub fn get(&self) -> u8 {
        return self._registers.borrow()[self._name as usize];
	}
	
	pub fn set(&mut self, new_value: u8) {
        self._registers.borrow_mut()[self._name as usize] = new_value;
	}
	
	pub fn overflowing_add(&self, other: u8) -> (u8, bool) {
        return self._registers.borrow_mut()[self._name as usize].overflowing_add(other);
	}
	
	pub fn overflowing_sub(&self, other: u8) -> (u8, bool) {
        return self._registers.borrow_mut()[self._name as usize].overflowing_sub(other);
	}
}

impl PartialEq<u8> for Register {
    fn eq(&self, other: &u8) -> bool {
        return self.get() == *other;
    }
}

impl fmt::UpperHex for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    
    #[test]
    fn test_set_sets_value() {
        const NEW_A: u8 = 0x12;
        let mut register = Register::new(Rc::new(RefCell::new([0; 8])), RegisterName::A);
		
		register.set(NEW_A);
        
        assert_eq!(as_hex!(register), as_hex!(NEW_A));
    }
    
    #[test]
    fn test_get_gets_value() {
        const NEW_A: u8 = 0x12;
        let mut register = Register::new(Rc::new(RefCell::new([0; 8])), RegisterName::A);
		
		register.set(NEW_A);
        
        assert_eq!(register.get(), NEW_A);
    }
	
    #[test]
    fn test_overflowing_add_no_overflow_returns_new_value_and_false() {
        const INTIAL_A: u8 = 0x12;
		const TO_ADD: u8 = 0x02;
		const EXPECTED_A: u8 = INTIAL_A + TO_ADD;
        
		let mut register = Register::new(Rc::new(RefCell::new([0; 8])), RegisterName::A);
		register.set(INTIAL_A);
		
		let result = register.overflowing_add(TO_ADD);
        
        assert_eq!(result, (EXPECTED_A, false));
    }
	
    #[test]
    fn test_overflowing_add_overflow_returns_wrapped_value_and_true() {
        const INTIAL_A: u8 = 0x12;
		const TO_ADD: u8 = 0xFF;
		const EXPECTED: (u8, bool) = INTIAL_A.overflowing_add(TO_ADD);
        
		let mut register = Register::new(Rc::new(RefCell::new([0; 8])), RegisterName::A);
		register.set(INTIAL_A);
		
		let result = register.overflowing_add(TO_ADD);
        
        assert_eq!(result, EXPECTED);
    }
	
    #[test]
    fn test_overflowing_sub_no_overflow_returns_new_value_and_false() {
        const INTIAL_A: u8 = 0x12;
		const TO_SUB: u8 = 0x02;
		const EXPECTED_A: u8 = INTIAL_A - TO_SUB;
        
		let mut register = Register::new(Rc::new(RefCell::new([0; 8])), RegisterName::A);
		register.set(INTIAL_A);
		
		let result = register.overflowing_sub(TO_SUB);
        
        assert_eq!(result, (EXPECTED_A, false));
    }
	
    #[test]
    fn test_overflowing_sub_overflow_returns_wrapped_value_and_true() {
        const INTIAL_A: u8 = 0x12;
		const TO_SUB: u8 = 0xFF;
		const EXPECTED: (u8, bool) = INTIAL_A.overflowing_sub(TO_SUB);
        
		let mut register = Register::new(Rc::new(RefCell::new([0; 8])), RegisterName::A);
		register.set(INTIAL_A);
		
		let result = register.overflowing_sub(TO_SUB);
        
        assert_eq!(result, EXPECTED);
    }
}
