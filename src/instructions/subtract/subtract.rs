use crate::registers::registers::Registers;
use crate::instructions::utils::half_carry_utils::{check_half_borrow};

pub fn subtract(registers: &mut Registers, value: u8) {
    let original_value = registers.a.get();
    let (new_value, overflow) = registers.a.overflowing_sub(value);
    
    registers.a.set(new_value);
    registers.zero_flag.set(new_value == 0);
    registers.subtract_flag.activate();
    registers.carry_flag.set(overflow);
    registers.half_carry_flag.set(check_half_borrow(original_value, value));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    
    #[test]
    fn test_subtract_subtracts_to_a_register() {
        const INITIAL_A: u8 = 0x34;
        const VALUE_TO_SUB: u8 = 0x12;
        const EXPECTED_VALUE: u8 = INITIAL_A - VALUE_TO_SUB;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        
        subtract(&mut registers, VALUE_TO_SUB);
        
        assert_eq!(as_hex!(registers.a), as_hex!(EXPECTED_VALUE));
    }
    
    #[test]
    fn test_subtract_becomes_zero_sets_zero_flag_to_true() {
        const INITIAL_A: u8 = 0x12;
        const VALUE_TO_SUB: u8 = INITIAL_A;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        
        subtract(&mut registers, VALUE_TO_SUB);
        
        assert_eq!(registers.zero_flag.get(), true);
    }
    
    #[test]
    fn test_subtract_becomes_non_zero_sets_zero_flag_to_false() {
        const INITIAL_A: u8 = 0x12;
        const VALUE_TO_SUB: u8 = 0xAB + INITIAL_A;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        
        subtract(&mut registers, VALUE_TO_SUB);
        
        assert_eq!(registers.zero_flag.get(), false);
    }
    
    #[test]
    fn test_subtract_turns_subtract_flag_on() {
        const INITIAL_A: u8 = 0x34;
        const VALUE_TO_SUB: u8 = 0x12;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.subtract_flag.set(true);
        
        subtract(&mut registers, VALUE_TO_SUB);
        
        assert_eq!(registers.subtract_flag.get(), true);
    }
    
    #[test]
    fn test_subtract_no_overflow_sets_carry_flag_off() {
        const INITIAL_A: u8 = 0xFF;
        const VALUE_TO_SUB: u8 = 0x00;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.carry_flag.set(true);
        
        subtract(&mut registers, VALUE_TO_SUB);
        
        assert_eq!(registers.carry_flag.get(), false);
    }
    
    #[test]
    fn test_subtract_overflowed_sets_carry_flag_on() {
        const INITIAL_A: u8 = 0x00;
        const VALUE_TO_SUB: u8 = 0x1;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.carry_flag.set(false);
        
        subtract(&mut registers, VALUE_TO_SUB);
        
        assert_eq!(registers.carry_flag.get(), true);
    }
    
    #[test]
    fn test_subtract_no_lower_nibble_overflow_sets_half_carry_flag_off() {
        const INITIAL_A: u8 = 0x20;
        const VALUE_TO_SUB: u8 = 0x10;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.half_carry_flag.set(true);
        
        subtract(&mut registers, VALUE_TO_SUB);
        
        assert_eq!(registers.half_carry_flag.get(), false);
    }
    
    #[test]
    fn test_subtract_lower_nibble_overflowed_sets_half_carry_flag_on() {
        const INITIAL_A: u8 = 0x10;
        const VALUE_TO_SUB: u8 = 0x1;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.half_carry_flag.set(false);
        
        subtract(&mut registers, VALUE_TO_SUB);
        
        assert_eq!(registers.half_carry_flag.get(), true);
    }
}