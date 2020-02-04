use super::super::super::registers::registers::Registers;

pub trait Source {
    fn read(&self, registers: &Registers) -> u8;
}
