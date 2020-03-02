use super::super::utils::{check_half_carry};
use super::super::super::instruction_context::InstructionContext;

pub fn inc(context: &mut InstructionContext, value: u8) -> u8 {
    let (new_value, overflow) = value.overflowing_add(1);
    
    context.registers_mut().zero_flag.set(new_value == 0);
    context.registers_mut().subtract_flag.reset();
    context.registers_mut().half_carry_flag.set(check_half_carry(value, 1));
    
    return new_value;
}

pub fn inc_short(context: &mut InstructionContext, value: u16) -> u16 {
    let (new_value, overflow) = value.overflowing_add(1);
    return new_value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    use crate::cpu::testing::build_test_instruction_context;
    
    #[test]
    fn test_inc_returns_incremented_value() {
        const VALUE: u8 = 0x12;
        const EXPECTED_VALUE: u8 = VALUE + 1;
        let mut context = build_test_instruction_context();
        
        let result = inc(&mut context, VALUE);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_VALUE));
    }
    
    #[test]
    fn test_inc_becomes_zero_sets_zero_flag_to_true() {
        const VALUE: u8 = 0xFF;
        let mut context = build_test_instruction_context();
        
        inc(&mut context, VALUE);
        
        assert_eq!(context.registers().zero_flag.get(), true);
    }
    
    #[test]
    fn test_inc_becomes_non_zero_sets_zero_flag_to_false() {
        const VALUE: u8 = 0x12;
        let mut context = build_test_instruction_context();
        
        inc(&mut context, VALUE);
        
        assert_eq!(context.registers().zero_flag.get(), false);
    }
    
    #[test]
    fn test_inc_turns_subtract_flag_off() {
        const VALUE: u8 = 0x12;
        let mut context = build_test_instruction_context();
        context.registers_mut().subtract_flag.set(true);
        
        inc(&mut context, VALUE);
        
        assert_eq!(context.registers().subtract_flag.get(), false);
    }
    
    #[test]
    fn test_inc_doesnt_change_carry_flag() {
        const VALUE: u8 = 0xFF;
        let mut context = build_test_instruction_context();
        context.registers_mut().carry_flag.set(true);
        
        inc(&mut context, VALUE);
        
        assert_eq!(context.registers().carry_flag.get(), true);
    }
    
    #[test]
    fn test_inc_no_lower_nibble_overflow_sets_half_carry_flag_off() {
        const VALUE: u8 = 0x12;
        let mut context = build_test_instruction_context();
        context.registers_mut().half_carry_flag.set(true);
        
        inc(&mut context, VALUE);
        
        assert_eq!(context.registers().half_carry_flag.get(), false);
    }
    
    #[test]
    fn test_inc_lower_nibble_overflowed_sets_half_carry_flag_on() {
        const VALUE: u8 = 0x1F;
        let mut context = build_test_instruction_context();
        context.registers_mut().half_carry_flag.set(false);
        
        inc(&mut context, VALUE);
        
        assert_eq!(context.registers().half_carry_flag.get(), true);
    }
    
    #[test]
    fn test_inc_short_returns_incremented_value() {
        const VALUE: u16 = 0x1234;
        const EXPECTED_VALUE: u16 = VALUE + 1;
        let mut context = build_test_instruction_context();
        
        let result = inc_short(&mut context, VALUE);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_VALUE));
    }
    
    #[test]
    fn test_inc_short_doesnt_change_zero_flag() {
        const VALUE: u16 = 0xFF;
        let mut context = build_test_instruction_context();
        context.registers_mut().zero_flag.set(true);
        
        inc_short(&mut context, VALUE);
        
        assert_eq!(context.registers().zero_flag.get(), true);
    }
    
    #[test]
    fn test_inc_short_doesnt_change_subtract_flag() {
        const VALUE: u16 = 0xFF;
        let mut context = build_test_instruction_context();
        context.registers_mut().subtract_flag.set(true);
        
        inc_short(&mut context, VALUE);
        
        assert_eq!(context.registers().subtract_flag.get(), true);
    }
    
    #[test]
    fn test_inc_short_doesnt_change_carry_flag() {
        const VALUE: u16 = 0xFF;
        let mut context = build_test_instruction_context();
        context.registers_mut().carry_flag.set(true);
        
        inc_short(&mut context, VALUE);
        
        assert_eq!(context.registers().carry_flag.get(), true);
    }
    
    #[test]
    fn test_inc_short_doesnt_change_half_carry_flag() {
        const VALUE: u16 = 0xFF;
        let mut context = build_test_instruction_context();
        context.registers_mut().half_carry_flag.set(true);
        
        inc_short(&mut context, VALUE);
        
        assert_eq!(context.registers().half_carry_flag.get(), true);
    }
}