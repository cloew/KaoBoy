use super::super::instruction::Instruction;
use super::super::sources::{ByteSource, RegisterSource};
use super::super::super::InstructionContext;
use super::super::super::registers::RegisterName;

pub type JumpConditionFn = fn(&InstructionContext) -> bool;

pub struct JumpInstruction {
    source: Box<dyn ByteSource>,
    condition: JumpConditionFn,
}

impl JumpInstruction {
	pub fn new(
            source: Box<dyn ByteSource>,
            condition: JumpConditionFn) -> JumpInstruction {
		return JumpInstruction {
            source: source,
            condition: condition,
        };
	}
}

impl Instruction for JumpInstruction {
	fn run(&self, context: &mut InstructionContext) {
        let relative_address = self.source.read(context) as i8;
        let new_counter = context.program_mut().get_counter().wrapping_add(relative_address as u16);
        if (self.condition)(context) {
            context.program_mut().set_counter(new_counter);
        }
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
    fn test_run_condition_true_jumps_to_relative_location() {
        const INITIAL_COUNTER: u16 = 0x0A + 1;
        const RELATIVE_JUMP: u8 = 0xFB;
        const EXPECTED_COUNTER: u16 = 0x07;
        let mut context = build_test_instruction_context();
        context.program_mut().set_counter(INITIAL_COUNTER);
        context.memory_mut().write_byte(INITIAL_COUNTER, RELATIVE_JUMP);
        let source = ConstantByteSource::new();
        
        let instruction = JumpInstruction::new(boxed!(source), always);
        instruction.run(&mut context);
        
        assert_eq!(context.program().get_counter(), EXPECTED_COUNTER);
    }
    
    #[test]
    fn test_run_condition_false_does_not_jump() {
        const INITIAL_COUNTER: u16 = 0x12;
        const RELATIVE_JUMP: u8 = 0x34;
        const EXPECTED_COUNTER: u16 = INITIAL_COUNTER + 1 + (RELATIVE_JUMP as u16);
        let mut context = build_test_instruction_context();
        context.program_mut().set_counter(INITIAL_COUNTER);
        context.memory_mut().write_byte(INITIAL_COUNTER, RELATIVE_JUMP);
        let source = ConstantByteSource::new();
        
        let instruction = JumpInstruction::new(boxed!(source), invalid_condition);
        instruction.run(&mut context);
        
        assert_eq!(context.program().get_counter(), INITIAL_COUNTER+1);
    }
}
