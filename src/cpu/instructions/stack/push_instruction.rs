use super::super::instruction::Instruction;
use super::super::sources::{ShortSource, DoubleRegisterSource};
use super::super::jump::{JumpConditionFn, jump_with_extra_work};
use super::super::super::registers::DoubleRegisterName;
use super::super::super::InstructionContext;
use crate::{boxed};

pub struct PushInstruction {
    source: Box<dyn ShortSource>,
}

impl PushInstruction {
	pub fn new(source: Box<dyn ShortSource>) -> PushInstruction {
		return PushInstruction {source: source};
	}
    
	pub fn new_for_register(register: DoubleRegisterName) -> PushInstruction {
		return PushInstruction {source: boxed!(DoubleRegisterSource::new(register))};
	}
}

impl Instruction for PushInstruction {
	fn run(&self, context: &mut InstructionContext) {
        let value = self.source.read(context);
        context.stack_mut().push(value);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::testing::build_test_instruction_context;
    use crate::{as_hex};
    
    #[test]
    fn test_run_pushes_value() {
        const EXPECTED_VALUE: u16 = 0x0A;
        let mut context = build_test_instruction_context();
        context.stack_mut().set_pointer(0xFFFE);
        context.registers_mut().hl.set(EXPECTED_VALUE);
        
        let instruction = PushInstruction::new_for_register(DoubleRegisterName::HL);
        instruction.run(&mut context);
        
        assert_eq!(as_hex!(context.stack_mut().pop()), as_hex!(EXPECTED_VALUE));
    }
}
