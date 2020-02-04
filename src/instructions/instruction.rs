use crate::registers::registers::Registers;

pub trait Instruction {
    fn run(&self, registers: &mut Registers);
}
