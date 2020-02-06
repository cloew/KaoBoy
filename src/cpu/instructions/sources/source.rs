use super::super::super::instruction_context::InstructionContext;

pub trait Source {
    fn read(&self, context: &InstructionContext) -> u8;
}
