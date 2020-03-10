use super::register_names::RegisterName;
use crate::{build_u16, get_lower_u8, get_upper_u8};

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct DoubleRegister {
	_registers: Rc<RefCell<[u8; 8]>>,
    _upper_name: RegisterName,
    _lower_name: RegisterName,
}

impl DoubleRegister {
	pub fn new(registers: Rc<RefCell<[u8; 8]>>, upper_register_name: RegisterName, lower_register_name: RegisterName) -> DoubleRegister {
		return DoubleRegister {_registers: registers, _upper_name: upper_register_name, _lower_name: lower_register_name};
	}
	
	pub fn get(&self) -> u16 {
        let upper_value = self.get_value(self._upper_name);
        let lower_value = self.get_value(self._lower_name);
        return build_u16!(upper_value, lower_value);
	}
	
	pub fn set(&mut self, new_value: u16) {
        self.set_value(self._upper_name, get_upper_u8!(new_value));
        self.set_value(self._lower_name, get_lower_u8!(new_value));
	}
	
	pub fn increment(&mut self) {
        let current_value = self.get();
        self.set(current_value+1);
	}
	
	pub fn decrement(&mut self) {
        let current_value = self.get();
        self.set(current_value-1);
	}
    
    fn get_value(&self, name: RegisterName) -> u8 {
        return self._registers.borrow()[name as usize];
    }
    
    fn set_value(&mut self, name: RegisterName, new_value: u8) {
        self._registers.borrow_mut()[name as usize] = new_value;
    }
}

impl PartialEq<u16> for DoubleRegister {
    fn eq(&self, other: &u16) -> bool {
        return self.get() == *other;
    }
}

impl fmt::UpperHex for DoubleRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    
    #[test]
    fn test_set_sets_value() {
        const EXPECTED: u16 = 0x1234;
        let mut double_register = DoubleRegister::new(Rc::new(RefCell::new([0; 8])), RegisterName::H, RegisterName::L);
		
		double_register.set(EXPECTED);
        
        assert_eq!(as_hex!(double_register), as_hex!(EXPECTED));
    }
    
    #[test]
    fn test_get_gets_value() {
        const NEW_H: u8 = 0x12;
        const NEW_L: u8 = 0x34;
        const EXPECTED: u16 = build_u16!(NEW_H, NEW_L);
        let mut registers = Rc::new(RefCell::new([0; 8]));
        let mut double_register = DoubleRegister::new(registers.clone(), RegisterName::H, RegisterName::L);
		registers.borrow_mut()[RegisterName::H as usize] = NEW_H;
        registers.borrow_mut()[RegisterName::L as usize] = NEW_L;
        
        let result = double_register.get();
        
        assert_eq!(result, EXPECTED);
    }
}
