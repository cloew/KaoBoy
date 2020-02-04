use super::register_names::RegisterName;
use std::rc::Rc;
use std::cell::RefCell;

pub struct RegisterFlag {
	_registers: Rc<RefCell<[u8; 8]>>,
    _name: RegisterName,
    mask: u8,
}

impl RegisterFlag {
	pub fn new(registers: Rc<RefCell<[u8; 8]>>, register_name: RegisterName, mask: u8) -> RegisterFlag {
		return RegisterFlag {_registers: registers, _name: register_name, mask: mask};
	}
	
	pub fn get(&self) -> bool {
        return self.get_flag_register() & self.mask > 0;
	}
	
	pub fn set(&mut self, set_to_true: bool) {
        if set_to_true {
            self._registers.borrow_mut()[self._name as usize] |= self.mask;
        } else {
            self._registers.borrow_mut()[self._name as usize] &= !self.mask;
        }
	}
    
    pub fn activate(&mut self) {
        self.set(true);
    }
    
    pub fn reset(&mut self) {
        self.set(false);
    }
	
	fn get_flag_register(&self) -> u8 {
        return self._registers.borrow()[self._name as usize];
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    
    const ALL_FLAGS_ON: u8 = 0xF0;
    const MASK: u8 = 0x80;
    
    #[test]
    fn test_get_flag_true_reads_flag_properly() {
        const INITIAL_FLAGS: u8 = MASK;
        const REGISTER: RegisterName = RegisterName::A;
        
        let register_flag = RegisterFlag::new(Rc::new(RefCell::new([0; 8])), REGISTER, MASK);
		register_flag._registers.borrow_mut()[REGISTER as usize] = INITIAL_FLAGS;
        
        let flag = register_flag.get();
        
        assert_eq!(flag, true);
    }
    
    #[test]
    fn test_get_flag_false_reads_flag_properly() {
        const INITIAL_FLAGS: u8 = ALL_FLAGS_ON & !MASK;
        const REGISTER: RegisterName = RegisterName::A;
        
        let register_flag = RegisterFlag::new(Rc::new(RefCell::new([0; 8])), REGISTER, MASK);
		register_flag._registers.borrow_mut()[REGISTER as usize] = INITIAL_FLAGS;
        
        let flag = register_flag.get();
        
        assert_eq!(flag, false);
    }
    
    #[test]
    fn test_set_flag_to_true_flags_only_the_masked_bit() {
        const INITIAL_FLAGS: u8 = ALL_FLAGS_ON & !MASK;
        const EXPECTED_FLAGS: u8 = ALL_FLAGS_ON;
        const REGISTER: RegisterName = RegisterName::A;
        
        let mut register_flag = RegisterFlag::new(Rc::new(RefCell::new([0; 8])), REGISTER, MASK);
		register_flag._registers.borrow_mut()[REGISTER as usize] = INITIAL_FLAGS;
        
        register_flag.set(true);
        
        assert_eq!(as_hex!(register_flag.get_flag_register()), as_hex!(EXPECTED_FLAGS));
    }
    
    #[test]
    fn test_set_flag_to_false_flags_only_the_masked_bit() {
        const INITIAL_FLAGS: u8 = ALL_FLAGS_ON;
        const EXPECTED_FLAGS: u8 = ALL_FLAGS_ON & !MASK;
        const REGISTER: RegisterName = RegisterName::A;
        
        let mut register_flag = RegisterFlag::new(Rc::new(RefCell::new([0; 8])), REGISTER, MASK);
		register_flag._registers.borrow_mut()[REGISTER as usize] = INITIAL_FLAGS;
        
        register_flag.set(false);
        
        assert_eq!(as_hex!(register_flag.get_flag_register()), as_hex!(EXPECTED_FLAGS));
    }
}
