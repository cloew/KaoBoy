use crate::instructions::instruction::{Instruction};
use crate::instructions::common::binary_byte_op::BinaryByteOp;
use crate::instructions::sources::register_source::RegisterSource;
use crate::instructions::xor::xor::{xor};
use crate::registers::register_names::{RegisterName};
use crate::{boxed, optional_boxed};

fn build_xor_instruction(other_source_name: RegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(BinaryByteOp::new_inplace_a_op(boxed!(RegisterSource::new(other_source_name)), xor));
}

pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        0xA8 => build_xor_instruction(RegisterName::B),
        0xA9 => build_xor_instruction(RegisterName::C),
        0xAA => build_xor_instruction(RegisterName::D),
        0xAB => build_xor_instruction(RegisterName::E),
        0xAC => build_xor_instruction(RegisterName::H),
        0xAD => build_xor_instruction(RegisterName::L),
        0xAF => build_xor_instruction(RegisterName::A),
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_instruction_returns_instruction() {
        const XOR_INSTRUCTION: u8 = 0x87;
        
        load_instruction(XOR_INSTRUCTION);
        
        // Not sure if there'e anything I can assert on
    }
}
