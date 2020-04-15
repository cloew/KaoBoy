use super::super::super::InstructionContext;

pub type JumpConditionFn = fn(&InstructionContext) -> bool;

pub fn jump(new_counter: u16, condition: JumpConditionFn, context: &mut InstructionContext) {
    if (condition)(context) {
        context.program_mut().set_counter(new_counter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::conditions::{always};
    use super::super::super::sources::ConstantByteSource;
    use crate::cpu::testing::build_test_instruction_context;
    use crate::{as_hex, boxed};
    
    fn invalid_condition(context: &InstructionContext) -> bool {
        return false;
    }
    
    #[test]
    fn test_run_condition_true_jumps_to_new_counter() {
        const INITIAL_COUNTER: u16 = 0x0A;
        const COUNTER_TO_JUMP_TO: u16 = 0x07;
        let mut context = build_test_instruction_context();
        context.program_mut().set_counter(INITIAL_COUNTER);
        
        jump(COUNTER_TO_JUMP_TO, always, &mut context);
        
        assert_eq!(context.program().get_counter(), COUNTER_TO_JUMP_TO);
    }
    
    #[test]
    fn test_run_condition_false_does_not_jump() {
        const INITIAL_COUNTER: u16 = 0x0A;
        const COUNTER_TO_JUMP_TO: u16 = 0x07;
        let mut context = build_test_instruction_context();
        context.program_mut().set_counter(INITIAL_COUNTER);
        
        jump(COUNTER_TO_JUMP_TO, invalid_condition, &mut context);
        
        assert_eq!(context.program().get_counter(), INITIAL_COUNTER);
    }
}
