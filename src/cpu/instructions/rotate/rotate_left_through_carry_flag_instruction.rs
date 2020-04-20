use super::super::instruction::Instruction;
use super::super::super::registers::RegisterName;
use super::super::super::InstructionContext;
use super::super::destinations::{ByteDestination, RegisterDestination};
use super::super::sources::{ByteSource, RegisterSource};
use crate::{boxed};

pub struct RotateLeftThroughCarryFlagInstruction {
    destination: Box<dyn ByteDestination>,
    source: Box<dyn ByteSource>,
}

impl RotateLeftThroughCarryFlagInstruction {
	pub fn new(destination: Box<dyn ByteDestination>, source: Box<dyn ByteSource>) -> RotateLeftThroughCarryFlagInstruction {
		return RotateLeftThroughCarryFlagInstruction {
            destination: destination,
            source: source
        };
	}
    
	pub fn new_for_register(register: RegisterName) -> RotateLeftThroughCarryFlagInstruction {
		return RotateLeftThroughCarryFlagInstruction {
            destination: boxed!(RegisterDestination::new(register)),
            source: boxed!(RegisterSource::new(register))
        };
	}
}

impl Instruction for RotateLeftThroughCarryFlagInstruction {
	fn run(&self, context: &mut InstructionContext) {
        let value = self.source.read(context);
        let original_carry_value = context.registers_mut().carry_flag.get();
        let mut new_value = value << 1;
        if (original_carry_value) {
            new_value += 1;
        }
        
        self.destination.assign(context, new_value);
        context.registers_mut().carry_flag.set((value >> 7) > 0);
        context.registers_mut().zero_flag.set(new_value == 0);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::testing::build_test_instruction_context;
    use crate::{as_hex};
    
    #[test]
    fn test_run_carry_flag_off_sets_new_register_and_carry_flag() {
        const INITIAL_VALUE: u8 = 0xFF;
        const EXPECTED_VALUE: u8 = INITIAL_VALUE << 1;
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(INITIAL_VALUE);
        context.registers_mut().carry_flag.reset();
        
        let instruction = RotateLeftThroughCarryFlagInstruction::new_for_register(RegisterName::A);
        instruction.run(&mut context);
        
        assert_eq!(as_hex!(context.registers_mut().a.get()), as_hex!(EXPECTED_VALUE));
        assert_eq!(context.registers_mut().carry_flag.get(), true);
    }
    
    #[test]
    fn test_run_carry_flag_on_sets_new_register_and_carry_flag() {
        const INITIAL_VALUE: u8 = 0x00;
        const EXPECTED_VALUE: u8 = 0x1;
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(INITIAL_VALUE);
        context.registers_mut().carry_flag.activate();
        
        let instruction = RotateLeftThroughCarryFlagInstruction::new_for_register(RegisterName::A);
        instruction.run(&mut context);
        
        assert_eq!(as_hex!(context.registers_mut().a.get()), as_hex!(EXPECTED_VALUE));
        assert_eq!(context.registers_mut().carry_flag.get(), false);
    }
    
    #[test]
    fn test_run_zero_activates_zero_flag() {
        const INITIAL_VALUE: u8 = 0x0;
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(INITIAL_VALUE);
        context.registers_mut().carry_flag.reset();
        
        let instruction = RotateLeftThroughCarryFlagInstruction::new_for_register(RegisterName::A);
        instruction.run(&mut context);
        
        assert_eq!(context.registers_mut().zero_flag.get(), true);
    }
    
    #[test]
    fn test_run_non_zero_resets_zero_flag() {
        const INITIAL_VALUE: u8 = 0x0;
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(INITIAL_VALUE);
        context.registers_mut().carry_flag.activate();
        
        let instruction = RotateLeftThroughCarryFlagInstruction::new_for_register(RegisterName::A);
        instruction.run(&mut context);
        
        assert_eq!(context.registers_mut().zero_flag.get(), false);
    }
}
