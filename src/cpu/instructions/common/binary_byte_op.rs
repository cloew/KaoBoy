use super::super::destinations::{ByteDestination, RegisterDestination};
use super::super::instruction::Instruction;
use super::super::sources::{ByteSource, RegisterSource};
use super::super::super::instruction_context::InstructionContext;
use super::super::super::registers::RegisterName;

type BinaryByteOpFn = fn(&mut InstructionContext, u8, u8) -> u8;

pub struct BinaryByteOp {
    left_source: Box<dyn ByteSource>,
    right_source: Box<dyn ByteSource>,
    op: BinaryByteOpFn,
    destination: Box<dyn ByteDestination>,
}

impl BinaryByteOp {
	pub fn new(
            left_source: Box<dyn ByteSource>,
            right_source: Box<dyn ByteSource>,
            op: BinaryByteOpFn,
            destination: Box<dyn ByteDestination>) -> BinaryByteOp {
		return BinaryByteOp {
            left_source: left_source,
            right_source: right_source,
            op: op,
            destination: destination,
        };
	}
    
	pub fn new_inplace_a_op(
            right_source: Box<dyn ByteSource>,
            op: BinaryByteOpFn,) -> BinaryByteOp {
		return BinaryByteOp {
            left_source: Box::new(RegisterSource::new(RegisterName::A)),
            right_source: right_source,
            op: op,
            destination: Box::new(RegisterDestination::new(RegisterName::A)),
        };
	}
}

impl Instruction for BinaryByteOp {
	fn run(&self, context: &mut InstructionContext) {
        let left_value = self.left_source.read(context);
        let right_value = self.right_source.read(context);
        let new_value = (self.op)(context, left_value, right_value);
        self.destination.assign(context, new_value);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    use crate::cpu::testing::build_test_instruction_context;
    
    fn fake_add_op(_context: &mut InstructionContext, left_value: u8, right_value: u8) -> u8 {
        return left_value + right_value;
    }
    
    #[test]
    fn test_run_calls_sources_op_and_destination() {
        const INITIAL_A: u8 = 0x12;
        const INITIAL_B: u8 = 0x56;
        const OP_RESULT: u8 = INITIAL_A + INITIAL_B;

        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(INITIAL_A);
        context.registers_mut().b.set(INITIAL_B);
        context.registers_mut().c.set(0x00);
    
        let left_source = RegisterSource::new(RegisterName::A);
        let right_source = RegisterSource::new(RegisterName::B);
        let destination = RegisterDestination::new(RegisterName::C);
        
        let instruction = BinaryByteOp::new(Box::new(left_source), Box::new(right_source), fake_add_op, Box::new(destination));
        
        instruction.run(&mut context);
        
        assert_eq!(as_hex!(context.registers().c), as_hex!(OP_RESULT));
    }
}
