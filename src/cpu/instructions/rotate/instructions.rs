use super::rotate_a_left::rotate_a_left;
use super::rotate_a_left_through_carry_flag::rotate_a_left_through_carry_flag;
use super::rotate_left::rotate_left;
use super::rotate_left_through_carry_flag::rotate_left_through_carry_flag;
use super::rotate_a_right::rotate_a_right;
use super::rotate_a_right_through_carry_flag::rotate_a_right_through_carry_flag;
use super::rotate_right::rotate_right;
use super::rotate_right_through_carry_flag::rotate_right_through_carry_flag;
use super::super::common::{UnaryByteOp, UnaryByteOpFn};
use super::super::sources::{AddressedByShortSource, RegisterSource};
use super::super::destinations::{AddressedByShortDestination, RegisterDestination};
use super::super::instruction::Instruction;
use super::super::super::registers::{DoubleRegisterName, RegisterName};
use crate::{boxed, optional_boxed};

fn build_rotate_register_instruction(register: RegisterName, operation: UnaryByteOpFn) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryByteOp::new(
            boxed!(RegisterSource::new(register)),
            operation,
            boxed!(RegisterDestination::new(register)))
    );
}

fn build_rotate_memory_instruction(register: DoubleRegisterName, operation: UnaryByteOpFn) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryByteOp::new(
            boxed!(AddressedByShortSource::new_from_register(register)),
            operation,
            boxed!(AddressedByShortDestination::new_from_register(register)))
    );
}

pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        // Rotate A Left
        0x07 => build_rotate_register_instruction(RegisterName::A, rotate_a_left),
        0x17 => build_rotate_register_instruction(RegisterName::A, rotate_a_left_through_carry_flag),
        // Rotate A Right
        0x0F => build_rotate_register_instruction(RegisterName::A, rotate_a_right),
        0x1F => build_rotate_register_instruction(RegisterName::A, rotate_a_right_through_carry_flag),
        _ => None,
    };
}

pub fn load_prefix_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        // Rotate Left
        0x00 => build_rotate_register_instruction(RegisterName::B, rotate_left),
        0x01 => build_rotate_register_instruction(RegisterName::C, rotate_left),
        0x02 => build_rotate_register_instruction(RegisterName::D, rotate_left),
        0x03 => build_rotate_register_instruction(RegisterName::E, rotate_left),
        0x04 => build_rotate_register_instruction(RegisterName::H, rotate_left),
        0x05 => build_rotate_register_instruction(RegisterName::L, rotate_left),
        0x06 => build_rotate_memory_instruction(DoubleRegisterName::HL, rotate_left),
        0x07 => build_rotate_register_instruction(RegisterName::A, rotate_left),
        
        // Rotate Left through Carry Flag Instructions
        0x10 => build_rotate_register_instruction(RegisterName::B, rotate_left_through_carry_flag),
        0x11 => build_rotate_register_instruction(RegisterName::C, rotate_left_through_carry_flag),
        0x12 => build_rotate_register_instruction(RegisterName::D, rotate_left_through_carry_flag),
        0x13 => build_rotate_register_instruction(RegisterName::E, rotate_left_through_carry_flag),
        0x14 => build_rotate_register_instruction(RegisterName::H, rotate_left_through_carry_flag),
        0x15 => build_rotate_register_instruction(RegisterName::L, rotate_left_through_carry_flag),
        0x16 => build_rotate_memory_instruction(DoubleRegisterName::HL, rotate_left_through_carry_flag),
        0x17 => build_rotate_register_instruction(RegisterName::A, rotate_left_through_carry_flag),
        
        // Rotate Right
        0x08 => build_rotate_register_instruction(RegisterName::B, rotate_right),
        0x09 => build_rotate_register_instruction(RegisterName::C, rotate_right),
        0x0A => build_rotate_register_instruction(RegisterName::D, rotate_right),
        0x0B => build_rotate_register_instruction(RegisterName::E, rotate_right),
        0x0C => build_rotate_register_instruction(RegisterName::H, rotate_right),
        0x0D => build_rotate_register_instruction(RegisterName::L, rotate_right),
        0x0E => build_rotate_memory_instruction(DoubleRegisterName::HL, rotate_right),
        0x0F => build_rotate_register_instruction(RegisterName::A, rotate_right),
        
        // Rotate Right through Carry Flag Instructions
        0x18 => build_rotate_register_instruction(RegisterName::B, rotate_right_through_carry_flag),
        0x19 => build_rotate_register_instruction(RegisterName::C, rotate_right_through_carry_flag),
        0x1A => build_rotate_register_instruction(RegisterName::D, rotate_right_through_carry_flag),
        0x1B => build_rotate_register_instruction(RegisterName::E, rotate_right_through_carry_flag),
        0x1C => build_rotate_register_instruction(RegisterName::H, rotate_right_through_carry_flag),
        0x1D => build_rotate_register_instruction(RegisterName::L, rotate_right_through_carry_flag),
        0x1E => build_rotate_memory_instruction(DoubleRegisterName::HL, rotate_right_through_carry_flag),
        0x1F => build_rotate_register_instruction(RegisterName::A, rotate_right_through_carry_flag),
        
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_prefix_instruction_returns_instruction() {
        const RL_INSTRUCTION: u8 = 0x10;
        
        load_prefix_instruction(RL_INSTRUCTION);
        
        // Not sure if there'e anything I can assert on
    }
}
