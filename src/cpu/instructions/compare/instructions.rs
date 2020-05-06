use super::compare_instruction::CompareInstruction;
use super::super::instruction::Instruction;
use super::super::sources::{AddressedByShortSource, ConstantByteSource};
use super::super::super::registers::{DoubleRegisterName, RegisterName};
use crate::{boxed, optional_boxed};

fn build_compare_instruction(source_name: RegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(CompareInstruction::new_for_register(source_name));
}

fn build_compare_instruction_from_memory() -> Option<Box<dyn Instruction>> {
    return optional_boxed!(CompareInstruction::new(boxed!(AddressedByShortSource::new_from_register(DoubleRegisterName::HL))));
}

fn build_compare_instruction_from_constant_byte() -> Option<Box<dyn Instruction>> {
    return optional_boxed!(CompareInstruction::new(boxed!(ConstantByteSource::new())));
}

pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        0xB8 => build_compare_instruction(RegisterName::B),
        0xB9 => build_compare_instruction(RegisterName::C),
        0xBA => build_compare_instruction(RegisterName::D),
        0xBB => build_compare_instruction(RegisterName::E),
        0xBC => build_compare_instruction(RegisterName::H),
        0xBD => build_compare_instruction(RegisterName::L),
        0xBE => build_compare_instruction_from_memory(),
        0xBF => build_compare_instruction(RegisterName::A),
        0xFE => build_compare_instruction_from_constant_byte(),
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_instruction_returns_instruction() {
        const CP_INSTRUCTION: u8 = 0xBE;
        
        load_instruction(CP_INSTRUCTION);
        
        // Not sure if there'e anything I can assert on
    }
}
