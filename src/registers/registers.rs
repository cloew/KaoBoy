use crate::{build_u16, get_lower_u8, get_upper_u8};
use crate::registers::register::{Register};
use crate::registers::register_names::{RegisterName};

use std::rc::Rc;
use std::cell::RefCell;

const HALF_CARRY_FLAG_MASK: u8 = 0x20;
const SUBTRACT_FLAG_MASK: u8 = 0x40;
const ZERO_FLAG_MASK: u8 = 0x80;

pub struct Registers {
    pub a: Register,
    pub b: Register,
    pub c: Register,
    pub d: Register,
    pub e: Register,
    pub f: u8, // Special Flags register
    pub h: Register,
    pub l: Register,
    
    pub _registers: Rc<RefCell<[u8; 8]>>,
}

impl Registers {
	pub fn new() -> Registers {
        let registers_ref = Rc::new(RefCell::new([0; 8]));
		return Registers {
            a: Register::new(registers_ref.clone(), RegisterName::A),
            b: Register::new(registers_ref.clone(), RegisterName::B),
            c: Register::new(registers_ref.clone(), RegisterName::C),
            d: Register::new(registers_ref.clone(), RegisterName::D),
            e: Register::new(registers_ref.clone(), RegisterName::E),
            f: 0,
            h: Register::new(registers_ref.clone(), RegisterName::H),
            l: Register::new(registers_ref.clone(), RegisterName::L),
            _registers: registers_ref};
	}
    
    pub fn get_zero_flag(&self) -> bool {
        return self.get_flag(ZERO_FLAG_MASK);
    }
    
    pub fn set_zero_flag(&mut self, is_zero: bool) {
        self.set_flag(ZERO_FLAG_MASK, is_zero);
    }
    
    pub fn get_subtract_flag(&self) -> bool {
        return self.get_flag(SUBTRACT_FLAG_MASK);
    }
    
    pub fn set_subtract_flag(&mut self, is_subtract: bool) {
        self.set_flag(SUBTRACT_FLAG_MASK, is_subtract);
    }
    
    pub fn get_half_carry_flag(&self) -> bool {
        return self.get_flag(HALF_CARRY_FLAG_MASK);
    }
    
    pub fn set_half_carry_flag(&mut self, is_half_carry: bool) {
        self.set_flag(HALF_CARRY_FLAG_MASK, is_half_carry);
    }
    fn get_flag(&self, flag_mask: u8) -> bool {
        return self.f & flag_mask > 0;
    }

    fn set_flag(&mut self, flag_mask: u8, set_to_true: bool) {
        if set_to_true {
            self.f |= flag_mask;
        } else {
            self.f &= !flag_mask;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    
    const ALL_FLAGS_ON: u8 = 0xF0;
    
    #[test]
    fn test_get_zero_flag_true_reads_flag_properly() {
        const INITIAL_FLAGS: u8 = ZERO_FLAG_MASK;
        
        let mut registers = Registers::new();
		registers.f = INITIAL_FLAGS;
        
        let zero_flag = registers.get_zero_flag();
        
        assert_eq!(zero_flag, true);
    }
    
    #[test]
    fn test_get_zero_flag_false_reads_flag_properly() {
        const INITIAL_FLAGS: u8 = ALL_FLAGS_ON & !ZERO_FLAG_MASK;
        
        let mut registers = Registers::new();
		registers.f = INITIAL_FLAGS;
        
        let zero_flag = registers.get_zero_flag();
        
        assert_eq!(zero_flag, false);
    }
    
    #[test]
    fn test_set_zero_flag_to_true_flags_only_the_zero_bit() {
        const INITIAL_FLAGS: u8 = ALL_FLAGS_ON & !ZERO_FLAG_MASK;
        const EXPECTED_FLAGS: u8 = ALL_FLAGS_ON;
        
        let mut registers = Registers::new();
		registers.f = INITIAL_FLAGS;
        
        registers.set_zero_flag(true);
        
        assert_eq!(as_hex!(registers.f), as_hex!(EXPECTED_FLAGS));
    }
    
    #[test]
    fn test_set_zero_flag_to_false_flags_only_the_zero_bit() {
        const INITIAL_FLAGS: u8 = ALL_FLAGS_ON;
        const EXPECTED_FLAGS: u8 = ALL_FLAGS_ON & !ZERO_FLAG_MASK;
        
        let mut registers = Registers::new();
		registers.f = INITIAL_FLAGS;
        
        registers.set_zero_flag(false);
        
        assert_eq!(as_hex!(registers.f), as_hex!(EXPECTED_FLAGS));
    }
    
    #[test]
    fn test_get_subtract_flag_true_reads_flag_properly() {
        const INITIAL_FLAGS: u8 = SUBTRACT_FLAG_MASK;
        
        let mut registers = Registers::new();
		registers.f = INITIAL_FLAGS;
        
        let subtract_flag = registers.get_subtract_flag();
        
        assert_eq!(subtract_flag, true);
    }
    
    #[test]
    fn test_get_subtract_flag_false_reads_flag_properly() {
        const INITIAL_FLAGS: u8 = ALL_FLAGS_ON & !SUBTRACT_FLAG_MASK;
        
        let mut registers = Registers::new();
		registers.f = INITIAL_FLAGS;
        
        let subtract_flag = registers.get_subtract_flag();
        
        assert_eq!(subtract_flag, false);
    }
    
    #[test]
    fn test_set_subtract_flag_to_true_flags_only_the_zero_bit() {
        const INITIAL_FLAGS: u8 = ALL_FLAGS_ON & !SUBTRACT_FLAG_MASK;
        const EXPECTED_FLAGS: u8 = ALL_FLAGS_ON;
        
        let mut registers = Registers::new();
		registers.f = INITIAL_FLAGS;
        
        registers.set_subtract_flag(true);
        
        assert_eq!(as_hex!(registers.f), as_hex!(EXPECTED_FLAGS));
    }
    
    #[test]
    fn test_set_subtract_flag_to_false_flags_only_the_zero_bit() {
        const INITIAL_FLAGS: u8 = ALL_FLAGS_ON;
        const EXPECTED_FLAGS: u8 = ALL_FLAGS_ON & !SUBTRACT_FLAG_MASK;
        
        let mut registers = Registers::new();
		registers.f = INITIAL_FLAGS;
        
        registers.set_subtract_flag(false);
        
        assert_eq!(as_hex!(registers.f), as_hex!(EXPECTED_FLAGS));
    }
    
    #[test]
    fn test_get_half_carry_flag_true_reads_flag_properly() {
        const INITIAL_FLAGS: u8 = HALF_CARRY_FLAG_MASK;
        
        let mut registers = Registers::new();
		registers.f = INITIAL_FLAGS;
        
        let half_carry_flag = registers.get_half_carry_flag();
        
        assert_eq!(half_carry_flag, true);
    }
    
    #[test]
    fn test_get_half_carry_flag_false_reads_flag_properly() {
        const INITIAL_FLAGS: u8 = ALL_FLAGS_ON & !HALF_CARRY_FLAG_MASK;
        
        let mut registers = Registers::new();
		registers.f = INITIAL_FLAGS;
        
        let half_carry_flag = registers.get_half_carry_flag();
        
        assert_eq!(half_carry_flag, false);
    }
    
    #[test]
    fn test_set_half_carry_flag_to_true_flags_only_the_zero_bit() {
        const INITIAL_FLAGS: u8 = ALL_FLAGS_ON & !HALF_CARRY_FLAG_MASK;
        const EXPECTED_FLAGS: u8 = ALL_FLAGS_ON;
        
        let mut registers = Registers::new();
		registers.f = INITIAL_FLAGS;
        
        registers.set_half_carry_flag(true);
        
        assert_eq!(as_hex!(registers.f), as_hex!(EXPECTED_FLAGS));
    }
    
    #[test]
    fn test_set_half_carry_flag_to_false_flags_only_the_zero_bit() {
        const INITIAL_FLAGS: u8 = ALL_FLAGS_ON;
        const EXPECTED_FLAGS: u8 = ALL_FLAGS_ON & !HALF_CARRY_FLAG_MASK;
        
        let mut registers = Registers::new();
		registers.f = INITIAL_FLAGS;
        
        registers.set_half_carry_flag(false);
        
        assert_eq!(as_hex!(registers.f), as_hex!(EXPECTED_FLAGS));
    }
}