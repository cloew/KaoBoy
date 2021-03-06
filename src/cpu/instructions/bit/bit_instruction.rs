use super::super::instruction::Instruction;
use super::super::sources::{ByteSource, RegisterSource};
use super::super::super::InstructionContext;
use super::super::super::registers::RegisterName;
use crate::{boxed};

pub struct BitInstruction {
    source: Box<dyn ByteSource>,
    bit_number: u8,
}

impl BitInstruction {
	pub fn new(
            source: Box<dyn ByteSource>,
            bit_number: u8) -> BitInstruction {
		return BitInstruction {
            source: source,
            bit_number: bit_number,
        };
	}
    
	pub fn new_for_register(
            register_name: RegisterName,
            bit_number: u8) -> BitInstruction {
        let source =  boxed!(RegisterSource::new(register_name));
		return BitInstruction {
            source: source,
            bit_number: bit_number,
        };
	}
}

impl Instruction for BitInstruction {
	fn run(&self, context: &mut InstructionContext) {
        let mask = 0x1 << self.bit_number;
        let value = self.source.read(context);
        let bit_is_off = value & mask == 0;
    
        context.registers_mut().zero_flag.set(bit_is_off);
        context.registers_mut().subtract_flag.reset();
        context.registers_mut().half_carry_flag.activate();
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::testing::build_test_instruction_context;
    use crate::{as_hex};
    
    #[test]
    fn test_run_bit_on_resets_zero_flag() {
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(0x10);
        
        let instruction = BitInstruction::new_for_register(RegisterName::A, 4);
        instruction.run(&mut context);
        
        assert_eq!(context.registers().zero_flag.get(), false);
    }
    
    #[test]
    fn test_run_bit_off_activates_zero_flag() {
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(0xEF);
        
        let instruction = BitInstruction::new_for_register(RegisterName::A, 4);
        instruction.run(&mut context);
        
        assert_eq!(context.registers().zero_flag.get(), true);
    }
    
    #[test]
    fn test_run_resets_subtract_flag() {
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(0x10);
        
        let instruction = BitInstruction::new_for_register(RegisterName::A, 4);
        instruction.run(&mut context);
        
        assert_eq!(context.registers().subtract_flag.get(), false);
    }
    
    #[test]
    fn test_run_activates_half_carry_flag() {
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(0x10);
        
        let instruction = BitInstruction::new_for_register(RegisterName::A, 4);
        instruction.run(&mut context);
        
        assert_eq!(context.registers().half_carry_flag.get(), true);
    }
}
