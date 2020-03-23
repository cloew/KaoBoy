use super::super::utils::{check_half_carry};
use super::super::super::instruction_context::InstructionContext;

pub fn is_carry_flag_off(context: &InstructionContext) -> bool {
    return context.registers().carry_flag.get() == false;
}

pub fn is_carry_flag_on(context: &InstructionContext) -> bool {
    return context.registers().carry_flag.get();
}

pub fn is_zero_flag_off(context: &InstructionContext) -> bool {
    return context.registers().zero_flag.get() == false;
}

pub fn is_zero_flag_on(context: &InstructionContext) -> bool {
    return context.registers().zero_flag.get();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    use crate::cpu::testing::build_test_instruction_context;
    
    #[test]
    fn test_is_carry_flag_off_flag_off_returns_true() {
        let mut context = build_test_instruction_context();
        context.registers_mut().carry_flag.reset();
        
        let result = is_carry_flag_off(&context);
        
        assert_eq!(result, true);
    }
    
    #[test]
    fn test_is_carry_flag_off_flag_on_returns_false() {
        let mut context = build_test_instruction_context();
        context.registers_mut().carry_flag.activate();
        
        let result = is_carry_flag_off(&context);
        
        assert_eq!(result, false);
    }
    
    #[test]
    fn test_is_carry_flag_on_flag_off_returns_false() {
        let mut context = build_test_instruction_context();
        context.registers_mut().carry_flag.reset();
        
        let result = is_carry_flag_on(&context);
        
        assert_eq!(result, false);
    }
    
    #[test]
    fn test_is_carry_flag_on_flag_on_returns_true() {
        let mut context = build_test_instruction_context();
        context.registers_mut().carry_flag.activate();
        
        let result = is_carry_flag_on(&context);
        
        assert_eq!(result, true);
    }
    
    #[test]
    fn test_is_zero_flag_off_flag_off_returns_true() {
        let mut context = build_test_instruction_context();
        context.registers_mut().zero_flag.reset();
        
        let result = is_zero_flag_off(&context);
        
        assert_eq!(result, true);
    }
    
    #[test]
    fn test_is_zero_flag_off_flag_on_returns_false() {
        let mut context = build_test_instruction_context();
        context.registers_mut().zero_flag.activate();
        
        let result = is_zero_flag_off(&context);
        
        assert_eq!(result, false);
    }
    
    #[test]
    fn test_is_zero_flag_on_flag_off_returns_false() {
        let mut context = build_test_instruction_context();
        context.registers_mut().zero_flag.reset();
        
        let result = is_zero_flag_on(&context);
        
        assert_eq!(result, false);
    }
    
    #[test]
    fn test_is_zero_flag_on_flag_on_returns_true() {
        let mut context = build_test_instruction_context();
        context.registers_mut().zero_flag.activate();
        
        let result = is_zero_flag_on(&context);
        
        assert_eq!(result, true);
    }
}