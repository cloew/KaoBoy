use super::conditions::{always, is_carry_flag_off, is_carry_flag_on, is_zero_flag_off, is_zero_flag_on};
use super::jump_instruction::{JumpConditionFn, JumpInstruction};
use super::super::instruction::Instruction;
use super::super::sources::{ByteSource, ConstantByteSource};
use crate::{boxed, optional_boxed};

fn build_relative_jump_instruction(source: Box<dyn ByteSource>, condition: JumpConditionFn) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        JumpInstruction::new(
            source,
            condition
        )
    );
}

pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        // Relative Jumps
        0x18 => build_relative_jump_instruction(boxed!(ConstantByteSource::new()), always),
        0x20 => build_relative_jump_instruction(boxed!(ConstantByteSource::new()), is_zero_flag_off),
        0x28 => build_relative_jump_instruction(boxed!(ConstantByteSource::new()), is_zero_flag_on),
        0x30 => build_relative_jump_instruction(boxed!(ConstantByteSource::new()), is_carry_flag_off),
        0x38 => build_relative_jump_instruction(boxed!(ConstantByteSource::new()), is_carry_flag_on),
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_instruction_returns_instruction() {
        const RELATIVE_JUMP_INSTRUCTION: u8 = 0x20;
        
        load_instruction(RELATIVE_JUMP_INSTRUCTION);
        
        // Not sure if there'e anything I can assert on
    }
}
