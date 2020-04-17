use super::super::instruction::Instruction;
use super::super::destinations::{DoubleRegisterDestination, ShortDestination};
use super::super::jump::{JumpConditionFn, jump_with_extra_work};
use super::super::super::registers::DoubleRegisterName;
use super::super::super::InstructionContext;
use crate::{boxed};

pub struct PopInstruction {
    destination: Box<dyn ShortDestination>,
}

impl PopInstruction {
	pub fn new(destination: Box<dyn ShortDestination>) -> PopInstruction {
		return PopInstruction {destination: destination};
	}
    
	pub fn new_for_register(register: DoubleRegisterName) -> PopInstruction {
		return PopInstruction {destination: boxed!(DoubleRegisterDestination::new(register))};
	}
}

impl Instruction for PopInstruction {
	fn run(&self, context: &mut InstructionContext) {
        let value = context.stack_mut().pop();
        self.destination.assign(context, value)
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::testing::build_test_instruction_context;
    use crate::{as_hex};
    
    #[test]
    fn test_run_pops_value() {
        const EXPECTED_VALUE: u16 = 0x0A;
        let mut context = build_test_instruction_context();
        context.stack_mut().set_pointer(0xFFFE);
        context.stack_mut().push(EXPECTED_VALUE);
        context.registers_mut().hl.set(0x0);
        
        let instruction = PopInstruction::new_for_register(DoubleRegisterName::HL);
        instruction.run(&mut context);
        
        assert_eq!(as_hex!(context.registers_mut().hl.get()), as_hex!(EXPECTED_VALUE));
    }
}
