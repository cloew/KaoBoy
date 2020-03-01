use super::super::destinations::ShortDestination;
use super::super::instruction::Instruction;
use super::super::sources::ShortSource;
use super::super::super::InstructionContext;
use super::no_op::short_no_op;

type UnaryShortOpFn = fn(&mut InstructionContext, u16) -> u16;

pub struct UnaryShortOp {
    source: Box<dyn ShortSource>,
    op: UnaryShortOpFn,
    destination: Box<dyn ShortDestination>,
}

impl UnaryShortOp {
	pub fn new(
            source: Box<dyn ShortSource>,
            op: UnaryShortOpFn,
            destination: Box<dyn ShortDestination>) -> UnaryShortOp {
		return UnaryShortOp {
            source: source,
            op: op,
            destination: destination,
        };
	}
    
	pub fn new_no_op(
            source: Box<dyn ShortSource>,
            destination: Box<dyn ShortDestination>) -> UnaryShortOp {
		return UnaryShortOp {
            source: source,
            op: short_no_op,
            destination: destination,
        };
	}
}

impl Instruction for UnaryShortOp {
	fn run(&self, context: &mut InstructionContext) {
        let current_value = self.source.read(context);
        let new_value = (self.op)(context, current_value);
        self.destination.assign(context, new_value);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::destinations::StackPointerDestination;
    use super::super::super::sources::ConstantShortSource;
    use crate::cpu::testing::build_test_instruction_context_with_memory;
    use crate::{as_hex, boxed, rc_refcell};
    use crate::emulator::Memory;
    
    fn fake_inc_op(_context: &mut InstructionContext, value: u16) -> u16 {
        return value + 1;
    }
    
    #[test]
    fn test_run_calls_source_op_and_destination() {
        const COUNTER: u16 = 0x01;
        const SOURCE_VALUE: u16 = 0x1234;
        const OP_RESULT: u16 = SOURCE_VALUE + 1;

        let memory = rc_refcell!(Memory::new());
        let mut context = build_test_instruction_context_with_memory(memory.clone());
        context.program_mut().set_counter(COUNTER);
        memory.borrow_mut().write_short(COUNTER, SOURCE_VALUE);
    
        let source = ConstantShortSource::new();
        let destination = StackPointerDestination::new();
        
        let instruction = UnaryShortOp::new(boxed!(source), fake_inc_op, boxed!(destination));
        
        instruction.run(&mut context);
        
        assert_eq!(as_hex!(context.stack_mut().get_pointer()), as_hex!(OP_RESULT));
    }
}
