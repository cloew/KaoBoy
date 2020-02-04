use super::super::destinations::destination::Destination;
use super::super::instruction::Instruction;
use super::super::sources::source::Source;
use super::super::super::registers::registers::Registers;

type UnaryByteOpFn = fn(&mut Registers, u8) -> u8;

pub struct UnaryByteOp {
    source: Box<dyn Source>,
    op: UnaryByteOpFn,
    destination: Box<dyn Destination>,
}

impl UnaryByteOp {
	pub fn new(
            source: Box<dyn Source>,
            op: UnaryByteOpFn,
            destination: Box<dyn Destination>) -> UnaryByteOp {
		return UnaryByteOp {
            source: source,
            op: op,
            destination: destination,
        };
	}
}

impl Instruction for UnaryByteOp {
	fn run(&self, registers: &mut Registers) {
        let current_value = self.source.read(registers);
        let new_value = (self.op)(registers, current_value);
        self.destination.assign(registers, new_value);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::destinations::register_destination::RegisterDestination;
    use super::super::super::sources::register_source::RegisterSource;
    use super::super::super::super::registers::register_names::RegisterName;
    use crate::{as_hex, boxed};
    
    fn fake_inc_op(_registers: &mut Registers, value: u8) -> u8 {
        return value + 1;
    }
    
    #[test]
    fn test_run_calls_source_op_and_destination() {
        const INITIAL_A: u8 = 0x12;
        const OP_RESULT: u8 = INITIAL_A + 1;

        let mut registers = Registers::new();
        registers.a.set(INITIAL_A);
        registers.c.set(0x00);
    
        let source = RegisterSource::new(RegisterName::A);
        let destination = RegisterDestination::new(RegisterName::C);
        
        let instruction = UnaryByteOp::new(boxed!(source), fake_inc_op, boxed!(destination));
        
        instruction.run(&mut registers);
        
        assert_eq!(as_hex!(registers.c), as_hex!(OP_RESULT));
    }
}
