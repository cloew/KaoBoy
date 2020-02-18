use super::super::destinations::ByteDestination;
use super::super::instruction::Instruction;
use super::super::sources::ByteSource;
use super::super::super::instruction_context::InstructionContext;

type UnaryByteOpFn = fn(&mut InstructionContext, u8) -> u8;

pub struct UnaryByteOp {
    source: Box<dyn ByteSource>,
    op: UnaryByteOpFn,
    destination: Box<dyn ByteDestination>,
}

impl UnaryByteOp {
	pub fn new(
            source: Box<dyn ByteSource>,
            op: UnaryByteOpFn,
            destination: Box<dyn ByteDestination>) -> UnaryByteOp {
		return UnaryByteOp {
            source: source,
            op: op,
            destination: destination,
        };
	}
}

impl Instruction for UnaryByteOp {
	fn run(&self, context: &mut InstructionContext) {
        let current_value = self.source.read(context);
        let new_value = (self.op)(context, current_value);
        self.destination.assign(context, new_value);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::destinations::register_destination::RegisterDestination;
    use super::super::super::sources::register_source::RegisterSource;
    use super::super::super::super::registers::register_names::RegisterName;
    use crate::cpu::testing::build_test_instruction_context;
    use crate::{as_hex, boxed};
    
    fn fake_inc_op(_context: &mut InstructionContext, value: u8) -> u8 {
        return value + 1;
    }
    
    #[test]
    fn test_run_calls_source_op_and_destination() {
        const INITIAL_A: u8 = 0x12;
        const OP_RESULT: u8 = INITIAL_A + 1;

        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(INITIAL_A);
        context.registers_mut().c.set(0x00);
    
        let source = RegisterSource::new(RegisterName::A);
        let destination = RegisterDestination::new(RegisterName::C);
        
        let instruction = UnaryByteOp::new(boxed!(source), fake_inc_op, boxed!(destination));
        
        instruction.run(&mut context);
        
        assert_eq!(as_hex!(context.registers().c), as_hex!(OP_RESULT));
    }
}
