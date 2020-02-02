use crate::instructions::instruction::{Instruction};
use crate::instructions::add_from_register::{AddFromRegister};
use crate::registers::register_names::{RegisterName};
use crate::{as_hex};

pub fn load_instruction(instruction_byte: u8) -> Box<dyn Instruction> {
    let instruction = match instruction_byte {
        0x87 => AddFromRegister::new(RegisterName::A),
        _ => panic!("Unknown instruction: {}", as_hex!(instruction_byte)),
    };
    
    return Box::new(instruction);
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
