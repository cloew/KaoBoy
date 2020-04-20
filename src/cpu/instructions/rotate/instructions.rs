use super::rotate_left_through_carry_flag::rotate_left_through_carry_flag;
use super::super::common::{UnaryByteOp};
use super::super::sources::{AddressedByShortSource, RegisterSource};
use super::super::destinations::{AddressedByDoubleRegisterDestination, RegisterDestination};
use super::super::instruction::Instruction;
use super::super::super::registers::RegisterName;
use crate::{boxed, optional_boxed};

fn build_rotate_left_through_carry_flag_for_register_instruction(register: RegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryByteOp::new(
            boxed!(RegisterSource::new(register)),
            rotate_left_through_carry_flag,
            boxed!(RegisterDestination::new(register)))
    );
}

pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        _ => None,
    };
}

pub fn load_prefix_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        // Rotate through Carry Flag Instructions
        0x10 => build_rotate_left_through_carry_flag_for_register_instruction(RegisterName::B),
        0x11 => build_rotate_left_through_carry_flag_for_register_instruction(RegisterName::C),
        0x12 => build_rotate_left_through_carry_flag_for_register_instruction(RegisterName::D),
        0x13 => build_rotate_left_through_carry_flag_for_register_instruction(RegisterName::E),
        0x14 => build_rotate_left_through_carry_flag_for_register_instruction(RegisterName::H),
        0x15 => build_rotate_left_through_carry_flag_for_register_instruction(RegisterName::L),
        //0x16 => build_rotate_left_through_carry_flag_instruction(RegisterName::B),
        0x17 => build_rotate_left_through_carry_flag_for_register_instruction(RegisterName::A),
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
