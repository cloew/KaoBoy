use super::add::{add};
use super::super::common::binary_byte_op::BinaryByteOp;
use super::super::instruction::Instruction;
use super::super::sources::constant_byte_source::ConstantByteSource;
use super::super::sources::register_source::RegisterSource;
use super::super::super::registers::register_names::RegisterName;
use crate::{boxed, optional_boxed};

fn build_add_instruction(other_source_name: RegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(BinaryByteOp::new_inplace_a_op(boxed!(RegisterSource::new(other_source_name)), add));
}

fn build_add_instruction_from_constant_byte() -> Option<Box<dyn Instruction>> {
    return optional_boxed!(BinaryByteOp::new_inplace_a_op(boxed!(ConstantByteSource::new()), add));
}

pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        0x80 => build_add_instruction(RegisterName::B),
        0x81 => build_add_instruction(RegisterName::C),
        0x82 => build_add_instruction(RegisterName::D),
        0x83 => build_add_instruction(RegisterName::E),
        0x84 => build_add_instruction(RegisterName::H),
        0x85 => build_add_instruction(RegisterName::L),
        0x87 => build_add_instruction(RegisterName::A),
        0xC6 => build_add_instruction_from_constant_byte(),
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_instruction_returns_instruction() {
        const ADD_INSTRUCTION: u8 = 0x87;
        
        load_instruction(ADD_INSTRUCTION);
        
        // Not sure if there'e anything I can assert on
    }
}
