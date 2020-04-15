use super::super::jump::{JumpConditionFn, always, is_carry_flag_off, is_carry_flag_on, is_zero_flag_off, is_zero_flag_on};
use super::call_instruction::CallInstruction;
use super::super::instruction::Instruction;
use super::super::sources::{ByteSource, ConstantByteSource};
use crate::{boxed, optional_boxed};

fn build_call_instruction(condition: JumpConditionFn) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        CallInstruction::new(
            condition
        )
    );
}

pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        // Call Instructions
        0xCD => build_call_instruction(always),
        0xC4 => build_call_instruction(is_zero_flag_off),
        0xCC => build_call_instruction(is_zero_flag_on),
        0xD4 => build_call_instruction(is_carry_flag_off),
        0xDC => build_call_instruction(is_carry_flag_on),
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_instruction_returns_instruction() {
        const CALL_INSTRUCTION: u8 = 0xCD;
        
        load_instruction(CALL_INSTRUCTION);
        
        // Not sure if there'e anything I can assert on
    }
}
