use super::super::instruction::Instruction;
use super::super::jump::{JumpConditionFn, jump_with_extra_work};
use super::super::super::InstructionContext;

pub struct CallInstruction {
    condition: JumpConditionFn,
}

impl CallInstruction {
	pub fn new(condition: JumpConditionFn) -> CallInstruction {
		return CallInstruction {condition: condition};
	}
}

impl Instruction for CallInstruction {
	fn run(&self, context: &mut InstructionContext) {
        let new_counter = context.program_mut().read_next_short();
        let current_counter = context.program_mut().get_counter();
        
        jump_with_extra_work(new_counter, self.condition, context, |context_again| {
            context_again.stack_mut().push(current_counter);
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
        context.program_mut().set_counter(INITIAL_COUNTER);
        context.memory_mut().write_short(INITIAL_COUNTER, COUNTER_TO_JUMP_TO);
        let source = ConstantByteSource::new();
        
        let instruction = CallInstruction::new(always);
        instruction.run(&mut context);
        
        assert_eq!(context.program().get_counter(), COUNTER_TO_JUMP_TO);
    }
    
    #[test]
    fn test_run_condition_false_does_not_jump() {
        const INITIAL_COUNTER: u16 = 0x12;
        const COUNTER_AFTER_READING_NEW_ADDRESS: u16 = INITIAL_COUNTER + 2;
        const COUNTER_TO_JUMP_TO: u16 = 0xCAB0;
        let mut context = build_test_instruction_context();
        context.stack_mut().set_pointer(0xFFFE);
        context.program_mut().set_counter(INITIAL_COUNTER);
        context.memory_mut().write_short(INITIAL_COUNTER, COUNTER_TO_JUMP_TO);
        let source = ConstantByteSource::new();
        
        let instruction = CallInstruction::new(invalid_condition);
        instruction.run(&mut context);
        
        assert_eq!(context.program().get_counter(), COUNTER_AFTER_READING_NEW_ADDRESS);
    }
    
    #[test]
    fn test_run_condition_true_stores_old_next_program_address_on_stack() {
        const INITIAL_COUNTER: u16 = 0x12;
        const EXPECTED_COUNTER_ON_STACK: u16 = INITIAL_COUNTER + 2;
        const COUNTER_TO_JUMP_TO: u16 = 0xCAB0;
        let mut context = build_test_instruction_context();
        context.stack_mut().set_pointer(0xFFFE);
        context.program_mut().set_counter(INITIAL_COUNTER);
        context.memory_mut().write_short(INITIAL_COUNTER, COUNTER_TO_JUMP_TO);
        let source = ConstantByteSource::new();
        
        let instruction = CallInstruction::new(always);
        instruction.run(&mut context);
        
        assert_eq!(context.stack_mut().pop(), EXPECTED_COUNTER_ON_STACK);
    }
    
    #[test]
    fn test_run_condition_false_doesnt_affect_stack() {
        const INITIAL_COUNTER: u16 = 0x12;
        const COUNTER_TO_JUMP_TO: u16 = 0xCAB0;
        let mut context = build_test_instruction_context();
        context.stack_mut().set_pointer(0xFFFE);
        context.program_mut().set_counter(INITIAL_COUNTER);
        context.memory_mut().write_short(INITIAL_COUNTER, COUNTER_TO_JUMP_TO);
        let source = ConstantByteSource::new();
        let stack_pointer_before = context.stack().get_pointer();
        
        let instruction = CallInstruction::new(invalid_condition);
        instruction.run(&mut context);
        
        assert_eq!(context.stack().get_pointer(), stack_pointer_before);
    }
}
