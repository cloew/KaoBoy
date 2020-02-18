use super::super::super::instruction_context::InstructionContext;

pub trait ByteSource {
    fn read(&self, context: &InstructionContext) -> u8;
}
