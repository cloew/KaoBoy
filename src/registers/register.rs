use std::fmt;

pub struct Register {
	value: u8
}

impl Register {
	pub fn new() -> Register {
		return Register {value: 0};
	}
	
	pub fn set(&mut self, new_value: u8) {
		self.value = new_value;
	}
	
	pub fn overflowing_add(&self, other: u8) -> (u8, bool) {
		return self.value.overflowing_add(other);
	}
}

impl PartialEq<u8> for Register {
    fn eq(&self, other: &u8) -> bool {
        return self.value == *other;
    }
}

impl fmt::UpperHex for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.value)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    
    #[test]
    fn test_set_sets_value() {
        const NEW_A: u8 = 0x12;
        let mut register = Register::new();
		
		register.set(NEW_A);
        
        assert_eq!(as_hex!(register), as_hex!(NEW_A));
    }
	
    #[test]
    fn test_overflowing_add_no_overflow_returns_new_value_and_false() {
        const INTIAL_A: u8 = 0x12;
		const TO_ADD: u8 = 0x02;
		const EXPECTED_A: u8 = INTIAL_A + TO_ADD;
        
		let mut register = Register::new();
		register.set(INTIAL_A);
		
		let result = register.overflowing_add(TO_ADD);
        
        assert_eq!(result, (EXPECTED_A, false));
    }
	
    #[test]
    fn test_overflowing_add_overflow_returns_wrapped_value_and_true() {
        const INTIAL_A: u8 = 0x12;
		const TO_ADD: u8 = 0xFF;
		const EXPECTED: (u8, bool) = INTIAL_A.overflowing_add(TO_ADD);
        
		let mut register = Register::new();
		register.set(INTIAL_A);
		
		let result = register.overflowing_add(TO_ADD);
        
        assert_eq!(result, EXPECTED);
    }
}
