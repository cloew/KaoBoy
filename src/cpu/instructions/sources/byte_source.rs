use super::super::super::InstructionContext;

pub trait ByteSource {
    fn read(&self, context: &InstructionContext) -> u8;
}
