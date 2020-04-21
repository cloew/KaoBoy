use super::super::instruction::Instruction;
use super::super::super::registers::RegisterName;
use super::super::super::InstructionContext;
use super::super::destinations::{ByteDestination, RegisterDestination};
use super::super::sources::{ByteSource, RegisterSource};
use crate::{boxed};

pub fn rotate_a_left(context: &mut InstructionContext, value: u8) -> u8 {
    let original_carry_value = context.registers_mut().carry_flag.get();
    let mut new_value = (value << 1) + (value >> 7);
    
    context.registers_mut().carry_flag.set((value >> 7) > 0);
    context.registers_mut().zero_flag.reset();
    context.registers_mut().half_carry_flag.reset();
    context.registers_mut().subtract_flag.reset();
    
    return new_value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::testing::build_test_instruction_context;
    use crate::{as_hex};
    
    #[test]
    fn test_run_carry_flag_off_sets_new_register_and_carry_flag() {
        const INITIAL_VALUE: u8 = 0xFF;
        const EXPECTED_VALUE: u8 = (INITIAL_VALUE << 1) + (INITIAL_VALUE >> 7);
        let mut context = build_test_instruction_context();
        context.registers_mut().carry_flag.reset();
        
        let result = rotate_a_left(&mut context, INITIAL_VALUE);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_VALUE));
        assert_eq!(context.registers_mut().carry_flag.get(), true);
    }
    
    #[test]
    fn test_run_carry_flag_on_sets_new_register_and_carry_flag() {
        const INITIAL_VALUE: u8 = 0x01;
        const EXPECTED_VALUE: u8 = 0x1 << 1;
        let mut context = build_test_instruction_context();
        context.registers_mut().carry_flag.activate();
        
        let result = rotate_a_left(&mut context, INITIAL_VALUE);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_VALUE));
        assert_eq!(context.registers_mut().carry_flag.get(), false);
    }
    
    #[test]
    fn test_run_resets_zero_flag() {
        const INITIAL_VALUE: u8 = 0x0;
        let mut context = build_test_instruction_context();
        context.registers_mut().carry_flag.reset();
        
        rotate_a_left(&mut context, INITIAL_VALUE);
        
        assert_eq!(context.registers_mut().zero_flag.get(), false);
    }
}