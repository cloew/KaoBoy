use crate::instructions::instruction::{Instruction};
use crate::instructions::add::add_from_register::{AddFromRegister};
use crate::registers::register_names::{RegisterName};

pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        0x80 => Some(Box::new(AddFromRegister::new(RegisterName::B))),
        0x81 => Some(Box::new(AddFromRegister::new(RegisterName::C))),
        0x82 => Some(Box::new(AddFromRegister::new(RegisterName::D))),
        0x83 => Some(Box::new(AddFromRegister::new(RegisterName::E))),
        0x84 => Some(Box::new(AddFromRegister::new(RegisterName::H))),
        0x85 => Some(Box::new(AddFromRegister::new(RegisterName::L))),
        0x87 => Some(Box::new(AddFromRegister::new(RegisterName::A))),
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
