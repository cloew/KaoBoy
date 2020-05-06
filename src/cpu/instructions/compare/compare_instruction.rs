use super::super::instruction::Instruction;
use super::super::subtract::subtract;
use super::super::super::registers::RegisterName;
use super::super::sources::{ByteSource, RegisterSource};
use super::super::super::InstructionContext;
use crate::{boxed};

pub struct CompareInstruction {
    source: Box<dyn ByteSource>,
}

impl CompareInstruction {
	pub fn new(source: Box<dyn ByteSource>) -> CompareInstruction {
		return CompareInstruction {source: source};
	}

	pub fn new_for_register(name: RegisterName) -> CompareInstruction {
		return CompareInstruction::new(boxed!(RegisterSource::new(name)));
	}
}

impl Instruction for CompareInstruction {
	fn run(&self, context: &mut InstructionContext) {
        let a_value = context.registers().a.get();
        let source_value = self.source.read(context);
        subtract(context, a_value, source_value);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::jump::always;
    use super::super::super::sources::ConstantByteSource;
    use crate::cpu::testing::build_test_instruction_context;
    use crate::{as_hex, boxed};
    
    #[test]
    fn test_run_no_change_to_a_register() {
        const INITIAL_A: u8 = 0x34;
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(INITIAL_A);
        
        let instruction = CompareInstruction::new_for_register(RegisterName::A);
        instruction.run(&mut context);
        
        assert_eq!(as_hex!(context.registers().a.get()), as_hex!(INITIAL_A));
    }
    
    #[test]
    fn test_subtract_becomes_zero_sets_zero_flag_to_true() {
        const INITIAL_A: u8 = 0x34;
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(INITIAL_A);
        
        let instruction = CompareInstruction::new_for_register(RegisterName::A);
        instruction.run(&mut context);
        
        assert_eq!(context.registers().zero_flag.get(), true);
    }
    
    #[test]
    fn test_subtract_becomes_non_zero_sets_zero_flag_to_false() {
        const INITIAL_A: u8 = 0x34;
        const OTHER_VALUE: u8 = 0x12;
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(INITIAL_A);
        context.registers_mut().b.set(OTHER_VALUE);
        
        let instruction = CompareInstruction::new_for_register(RegisterName::B);
        instruction.run(&mut context);
        
        assert_eq!(context.registers().zero_flag.get(), false);
    }
    
    #[test]
    fn test_subtract_turns_subtract_flag_on() {
        const INITIAL_A: u8 = 0x34;
        const OTHER_VALUE: u8 = 0x12;
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(INITIAL_A);
        context.registers_mut().b.set(OTHER_VALUE);
        
        let instruction = CompareInstruction::new_for_register(RegisterName::B);
        instruction.run(&mut context);
        
        assert_eq!(context.registers().subtract_flag.get(), true);
    }
    
    #[test]
    fn test_subtract_no_overflow_sets_carry_flag_off() {
        const INITIAL_A: u8 = 0xFF;
        const OTHER_VALUE: u8 = 0x00;
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(INITIAL_A);
        context.registers_mut().b.set(OTHER_VALUE);
        
        let instruction = CompareInstruction::new_for_register(RegisterName::B);
        instruction.run(&mut context);
        
        assert_eq!(context.registers().carry_flag.get(), false);
    }
    
    #[test]
    fn test_subtract_overflowed_sets_carry_flag_on() {
        const INITIAL_A: u8 = 0x00;
        const OTHER_VALUE: u8 = 0x1;
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(INITIAL_A);
        context.registers_mut().b.set(OTHER_VALUE);
        
        let instruction = CompareInstruction::new_for_register(RegisterName::B);
        instruction.run(&mut context);
        
        assert_eq!(context.registers().carry_flag.get(), true);
    }
    
    #[test]
    fn test_subtract_no_lower_nibble_overflow_sets_half_carry_flag_off() {
        const INITIAL_A: u8 = 0x20;
        const OTHER_VALUE: u8 = 0x10;
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(INITIAL_A);
        context.registers_mut().b.set(OTHER_VALUE);
        
        let instruction = CompareInstruction::new_for_register(RegisterName::B);
        instruction.run(&mut context);
        
        assert_eq!(context.registers().half_carry_flag.get(), false);
    }
    
    #[test]
    fn test_subtract_lower_nibble_overflowed_sets_half_carry_flag_on() {
        const INITIAL_A: u8 = 0x10;
        const OTHER_VALUE: u8 = 0x1;
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(INITIAL_A);
        context.registers_mut().b.set(OTHER_VALUE);
        
        let instruction = CompareInstruction::new_for_register(RegisterName::B);
        instruction.run(&mut context);
        
        assert_eq!(context.registers().half_carry_flag.get(), true);
    }
}
