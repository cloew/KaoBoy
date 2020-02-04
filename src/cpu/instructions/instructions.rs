use super::add;
use super::load;
use super::subtract;
use super::xor;
use super::instruction::Instruction;
use crate::as_hex;

pub fn load_instruction(instruction_byte: u8) -> Box<dyn Instruction> {
    let package_instruction_loaders: Vec<fn(u8) -> 
            std::option::Option<std::boxed::Box<(dyn Instruction + 'static)>>>
            = vec![
        add::instructions::load_instruction,
        load::instructions::load_instruction,
        subtract::instructions::load_instruction,
        xor::instructions::load_instruction,
    ];
    
    let next_instruction = package_instruction_loaders.iter().find_map(
        |load_from_package| load_from_package(instruction_byte)
    );
    
    return match next_instruction {
        Some(instruction) => {
            instruction
        },
        None => panic!("Unknown instruction: {}", as_hex!(instruction_byte)),
    }
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
