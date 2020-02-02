use crate::cpu::memory::{Memory};
use crate::cpu::program_counter::{ProgramCounter};
use crate::registers::registers::Registers;

pub struct Cpu {
    _counter: ProgramCounter,
    _registers: Registers,
}

impl Cpu {
    pub fn new() -> Cpu {
        return Cpu {
            _counter: ProgramCounter::new(Memory::new()),
            _registers: Registers::new(),
        };
    }
}
