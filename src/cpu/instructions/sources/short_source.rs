use super::super::super::InstructionContext;

pub trait ShortSource {
    fn read(&self, context: &InstructionContext) -> u16;
}
