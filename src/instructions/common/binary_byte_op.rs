use crate::instructions::instruction::Instruction;
use crate::instructions::destinations::destination::Destination;
use crate::instructions::sources::source::Source;
use crate::registers::registers::Registers;

type BinaryByteOpFn = fn(&Registers, u8, u8) -> u8;

pub struct BinaryByteOp {
    left_source: Box<dyn Source>,
    right_source: Box<dyn Source>,
    op: fn(&Registers, u8, u8) -> u8,
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
    use crate::instructions::destinations::register_destination::RegisterDestination;
    use crate::instructions::sources::register_source::RegisterSource;
    use crate::registers::register_names::RegisterName;
    
    fn fake_add_op(registers: &Registers, left_value: u8, right_value: u8) -> u8 {
        return left_value + right_value;
    }
    
    fn fake_op_wrapper() -> BinaryByteOpFn {
        return |registers: &Registers, left_value: u8, right_value: u8| left_value + right_value;
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
    
        let mut left_source = RegisterSource::new(RegisterName::A);
        let mut right_source = RegisterSource::new(RegisterName::B);
        let destination = RegisterDestination::new(RegisterName::C);
        
        let instruction = BinaryByteOp::new(Box::new(left_source), Box::new(right_source), fake_add_op, Box::new(destination));
        
        instruction.run(&mut registers);
        
        assert_eq!(as_hex!(registers.c), as_hex!(OP_RESULT));
    }
}
