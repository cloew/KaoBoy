use super::super::instruction_context::InstructionContext;

pub trait Instruction {
    fn run(&self, context: &mut InstructionContext);
}
