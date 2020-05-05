use super::super::jump::{JumpConditionFn, always, is_carry_flag_off, is_carry_flag_on, is_zero_flag_off, is_zero_flag_on};
use super::call_instruction::CallInstruction;
use super::pop_instruction::PopInstruction;
use super::push_instruction::PushInstruction;
use super::return_instruction::ReturnInstruction;
use super::super::instruction::Instruction;
use super::super::sources::{ByteSource, ConstantByteSource};
use super::super::super::registers::DoubleRegisterName;
use crate::{boxed, optional_boxed};

fn build_call_instruction(condition: JumpConditionFn) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        CallInstruction::new(
            condition
        )
    );
}

fn build_pop_instruction(register: DoubleRegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        PopInstruction::new_for_register(
            register
        )
    );
}

fn build_push_instruction(register: DoubleRegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        PushInstruction::new_for_register(
            register
        )
    );
}

fn build_return_instruction(condition: JumpConditionFn) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        ReturnInstruction::new(
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
        // Pop Instructions
        0xC1 => build_pop_instruction(DoubleRegisterName::BC),
        0xD1 => build_pop_instruction(DoubleRegisterName::DE),
        0xE1 => build_pop_instruction(DoubleRegisterName::HL),
        0xF1 => build_pop_instruction(DoubleRegisterName::AF),
        // Push Instructions
        0xC5 => build_push_instruction(DoubleRegisterName::BC),
        0xD5 => build_push_instruction(DoubleRegisterName::DE),
        0xE5 => build_push_instruction(DoubleRegisterName::HL),
        0xF5 => build_push_instruction(DoubleRegisterName::AF),
        // Return Instructions
        0xC9 => build_return_instruction(always),
        0xC0 => build_return_instruction(is_zero_flag_off),
        0xC8 => build_return_instruction(is_zero_flag_on),
        0xD0 => build_return_instruction(is_carry_flag_off),
        0xD8 => build_return_instruction(is_carry_flag_on),
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
