use crate::instructions::instruction::{Instruction};
use crate::instructions::subtract::subtract_from_register::{SubtractFromRegister};
use crate::registers::register_names::{RegisterName};

pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        0x90 => Some(Box::new(SubtractFromRegister::new(RegisterName::B))),
        0x91 => Some(Box::new(SubtractFromRegister::new(RegisterName::C))),
        0x92 => Some(Box::new(SubtractFromRegister::new(RegisterName::D))),
        0x93 => Some(Box::new(SubtractFromRegister::new(RegisterName::E))),
        0x94 => Some(Box::new(SubtractFromRegister::new(RegisterName::H))),
        0x95 => Some(Box::new(SubtractFromRegister::new(RegisterName::L))),
        0x97 => Some(Box::new(SubtractFromRegister::new(RegisterName::A))),
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
