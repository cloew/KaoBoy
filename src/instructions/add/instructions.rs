use crate::instructions::instruction::{Instruction};
use crate::instructions::add::add_from_register::{AddFromRegister};
use crate::instructions::add::add::{add};
use crate::instructions::sources::register_source::RegisterSource;
use crate::registers::register_names::{RegisterName};
use crate::instructions::common::binary_byte_op::BinaryByteOp;

fn build_add_instruction(other_source_name: RegisterName) -> Option<Box<dyn Instruction>> {
    return Some(Box::new(BinaryByteOp::new_inplace_a_op(Box::new(RegisterSource::new(other_source_name)), add)));
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
