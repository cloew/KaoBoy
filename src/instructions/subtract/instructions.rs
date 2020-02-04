use super::subtract::subtract;
use super::super::instruction::Instruction;
use super::super::common::binary_byte_op::BinaryByteOp;
use super::super::sources::register_source::RegisterSource;
use crate::registers::register_names::RegisterName;
use crate::{boxed, optional_boxed};

fn build_subtract_instruction(other_source_name: RegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(BinaryByteOp::new_inplace_a_op(boxed!(RegisterSource::new(other_source_name)), subtract));
}

pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        0x90 => build_subtract_instruction(RegisterName::B),
        0x91 => build_subtract_instruction(RegisterName::C),
        0x92 => build_subtract_instruction(RegisterName::D),
        0x93 => build_subtract_instruction(RegisterName::E),
        0x94 => build_subtract_instruction(RegisterName::H),
        0x95 => build_subtract_instruction(RegisterName::L),
        0x97 => build_subtract_instruction(RegisterName::A),
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_instruction_returns_instruction() {
        const SUB_INSTRUCTION: u8 = 0x97;
        
        load_instruction(SUB_INSTRUCTION);
        
        // Not sure if there'e anything I can assert on
    }
}
