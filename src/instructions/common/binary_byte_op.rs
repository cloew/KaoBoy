use super::super::destinations::destination::Destination;
use super::super::destinations::register_destination::RegisterDestination;
use super::super::instruction::Instruction;
use super::super::sources::register_source::RegisterSource;
use super::super::sources::source::Source;
use super::super::super::registers::registers::Registers;
use crate::registers::register_names::RegisterName;

type BinaryByteOpFn = fn(&mut Registers, u8, u8) -> u8;

pub struct BinaryByteOp {
    left_source: Box<dyn Source>,
    right_source: Box<dyn Source>,
    op: BinaryByteOpFn,
    destination: Box<dyn Destination>,
}

impl BinaryByteOp {
	pub fn new(
            left_source: Box<dyn Source>,
            right_source: Box<dyn Source>,
            op: BinaryByteOpFn,
            destination: Box<dyn Destination>) -> BinaryByteOp {
		return BinaryByteOp {
            left_source: left_source,
            right_source: right_source,
            op: op,
            destination: destination,
        };
	}
    
	pub fn new_inplace_a_op(
            right_source: Box<dyn Source>,
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
	fn run(&self, registers: &mut Registers) {
        let left_value = self.left_source.read(registers);
        let right_value = self.right_source.read(registers);
        let new_value = (self.op)(registers, left_value, right_value);
        self.destination.assign(registers, new_value);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    
    fn fake_add_op(_registers: &mut Registers, left_value: u8, right_value: u8) -> u8 {
        return left_value + right_value;
    }
    
    #[test]
    fn test_run_calls_sources_op_and_destination() {
        const INITIAL_A: u8 = 0x12;
        const INITIAL_B: u8 = 0x56;
        const OP_RESULT: u8 = INITIAL_A + INITIAL_B;

        let mut registers = Registers::new();
        registers.a.set(INITIAL_A);
        registers.b.set(INITIAL_B);
        registers.c.set(0x00);
    
        let left_source = RegisterSource::new(RegisterName::A);
        let right_source = RegisterSource::new(RegisterName::B);
        let destination = RegisterDestination::new(RegisterName::C);
        
        let instruction = BinaryByteOp::new(Box::new(left_source), Box::new(right_source), fake_add_op, Box::new(destination));
        
        instruction.run(&mut registers);
        
        assert_eq!(as_hex!(registers.c), as_hex!(OP_RESULT));
    }
}
