use super::add;
use super::inc;
use super::load;
use super::subtract;
use super::xor;
use super::instruction::Instruction;
use super::super::ProgramCounter;
use crate::as_hex;

pub fn load_instruction(program: &mut ProgramCounter) -> Box<dyn Instruction> {
    let package_instruction_loaders: Vec<fn(u8) -> 
            std::option::Option<std::boxed::Box<(dyn Instruction + 'static)>>>
            = vec![
        add::instructions::load_instruction,
        inc::instructions::load_instruction,
        load::instructions::load_instruction,
        subtract::instructions::load_instruction,
        xor::instructions::load_instruction,
    ];
    
    let instruction_byte = program.read_next_byte();
    
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
    use crate::cpu::testing::build_test_program_counter;
    
    #[test]
    fn test_load_instruction_returns_instruction() {
        const ADD_INSTRUCTION: u8 = 0x87;
        let mut program = build_test_program_counter();
        program._memory.borrow_mut().write_byte(0x0000, ADD_INSTRUCTION);
        
        load_instruction(&mut program);
        
        // Not sure if there'e anything I can assert on
    }
}
