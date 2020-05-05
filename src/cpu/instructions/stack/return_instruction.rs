use super::super::instruction::Instruction;
use super::super::jump::{JumpConditionFn, jump_with_extra_work};
use super::super::super::InstructionContext;

pub struct ReturnInstruction {
    condition: JumpConditionFn,
}

impl ReturnInstruction {
	pub fn new(condition: JumpConditionFn) -> ReturnInstruction {
		return ReturnInstruction {condition: condition};
	}
}

impl Instruction for ReturnInstruction {
	fn run(&self, context: &mut InstructionContext) {
        jump_with_extra_work(self.condition, context, |context_again| {
            return context_again.stack_mut().pop();
        });
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::jump::always;
    use super::super::super::sources::ConstantByteSource;
    use crate::cpu::testing::build_test_instruction_context;
    use crate::{as_hex, boxed};
    
    fn invalid_condition(context: &InstructionContext) -> bool {
        return false;
    }
    
    #[test]
    fn test_run_condition_true_jumps_to_new_address() {
        const INITIAL_COUNTER: u16 = 0x0A;
        const COUNTER_TO_JUMP_TO: u16 = 0x07;
        let mut context = build_test_instruction_context();
        context.stack_mut().set_pointer(0xFFFE);
        context.stack_mut().push(COUNTER_TO_JUMP_TO);
        context.program_mut().set_counter(INITIAL_COUNTER);
        
        let instruction = ReturnInstruction::new(always);
        instruction.run(&mut context);
        
        assert_eq!(context.program().get_counter(), COUNTER_TO_JUMP_TO);
    }
    
    #[test]
    fn test_run_condition_false_does_not_jump() {
        const INITIAL_COUNTER: u16 = 0x12;
        const COUNTER_TO_JUMP_TO: u16 = 0xCAB0;
        let mut context = build_test_instruction_context();
        context.stack_mut().set_pointer(0xFFFE);
        context.stack_mut().push(COUNTER_TO_JUMP_TO);
        context.program_mut().set_counter(INITIAL_COUNTER);
        
        let instruction = ReturnInstruction::new(invalid_condition);
        instruction.run(&mut context);
        
        assert_eq!(context.program().get_counter(), INITIAL_COUNTER);
    }
    
    #[test]
    fn test_run_condition_true_extracts_old_program_address_from_stack() {
        const INITIAL_COUNTER: u16 = 0x12;
        const EXPECTED_COUNTER_ON_STACK: u16 = INITIAL_COUNTER - 2;
        const COUNTER_TO_JUMP_TO: u16 = 0xCAB0;
        const INITIAL_STACK_POINTER: u16 = 0x12;
        let mut context = build_test_instruction_context();
        context.stack_mut().set_pointer(INITIAL_STACK_POINTER);
        context.stack_mut().push(COUNTER_TO_JUMP_TO);
        context.program_mut().set_counter(INITIAL_COUNTER);
        
        let instruction = ReturnInstruction::new(always);
        instruction.run(&mut context);
        
        assert_eq!(context.stack_mut().get_pointer(), INITIAL_STACK_POINTER);
    }
    
    #[test]
    fn test_run_condition_false_doesnt_affect_stack() {
        const INITIAL_COUNTER: u16 = 0x12;
        const COUNTER_TO_JUMP_TO: u16 = 0xCAB0;
        let mut context = build_test_instruction_context();
        context.stack_mut().set_pointer(0xFFFE);
        context.stack_mut().push(COUNTER_TO_JUMP_TO);
        context.program_mut().set_counter(INITIAL_COUNTER);
        let stack_pointer_before = context.stack().get_pointer();
        
        let instruction = ReturnInstruction::new(invalid_condition);
        instruction.run(&mut context);
        
        assert_eq!(context.stack().get_pointer(), stack_pointer_before);
    }
}
