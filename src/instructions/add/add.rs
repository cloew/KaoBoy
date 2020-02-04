use super::super::utils::half_carry_utils::{check_half_carry};
use crate::registers::registers::Registers;

pub fn add(registers: &mut Registers, left_value: u8, right_value: u8) -> u8 {
    let (new_value, overflow) = left_value.overflowing_add(right_value);
    
    registers.zero_flag.set(new_value == 0);
    registers.subtract_flag.reset();
    registers.carry_flag.set(overflow);
    registers.half_carry_flag.set(check_half_carry(left_value, right_value));
    
    return new_value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    
    #[test]
    fn test_add_returns_added_value() {
        const LEFT_VALUE: u8 = 0x12;
        const RIGHT_VALUE: u8 = 0x34;
        const EXPECTED_VALUE: u8 = LEFT_VALUE + RIGHT_VALUE;
        let mut registers = Registers::new();
        
        let result = add(&mut registers, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_VALUE));
    }
    
    #[test]
    fn test_add_becomes_zero_sets_zero_flag_to_true() {
        const LEFT_VALUE: u8 = 0x12;
        const RIGHT_VALUE: u8 = (0x00 as u8).overflowing_sub(LEFT_VALUE).0;
        let mut registers = Registers::new();
        
        add(&mut registers, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(registers.zero_flag.get(), true);
    }
    
    #[test]
    fn test_add_becomes_non_zero_sets_zero_flag_to_false() {
        const LEFT_VALUE: u8 = 0x12;
        const RIGHT_VALUE: u8 = 0xFF - LEFT_VALUE;
        let mut registers = Registers::new();
        
        add(&mut registers, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(registers.zero_flag.get(), false);
    }
    
    #[test]
    fn test_add_turns_subtract_flag_off() {
        const LEFT_VALUE: u8 = 0x12;
        const RIGHT_VALUE: u8 = 0x34;
        
        let mut registers = Registers::new();
        registers.subtract_flag.set(true);
        
        add(&mut registers, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(registers.subtract_flag.get(), false);
    }
    
    #[test]
    fn test_add_no_overflow_sets_carry_flag_off() {
        const LEFT_VALUE: u8 = 0xFF;
        const RIGHT_VALUE: u8 = 0x00;
        
        let mut registers = Registers::new();
        registers.carry_flag.set(true);
        
        add(&mut registers, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(registers.carry_flag.get(), false);
    }
    
    #[test]
    fn test_add_overflowed_sets_carry_flag_on() {
        const LEFT_VALUE: u8 = 0xFF;
        const RIGHT_VALUE: u8 = 0x1;

        let mut registers = Registers::new();
        registers.carry_flag.set(false);
        
        add(&mut registers, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(registers.carry_flag.get(), true);
    }
    
    #[test]
    fn test_add_no_lower_nibble_overflow_sets_half_carry_flag_off() {
        const LEFT_VALUE: u8 = 0xB0;
        const RIGHT_VALUE: u8 = 0x10;
        
        let mut registers = Registers::new();
        registers.half_carry_flag.set(true);
        
        add(&mut registers, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(registers.half_carry_flag.get(), false);
    }
    
    #[test]
    fn test_add_lower_nibble_overflowed_sets_half_carry_flag_on() {
        const LEFT_VALUE: u8 = 0xFF;
        const RIGHT_VALUE: u8 = 0x1;
        
        let mut registers = Registers::new();
        registers.half_carry_flag.set(false);
        
        add(&mut registers, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(registers.half_carry_flag.get(), true);
    }
}