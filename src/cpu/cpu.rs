use super::memory::Memory;
use super::program_counter::ProgramCounter;
use super::registers::registers::Registers;

pub struct Cpu {
    _counter: ProgramCounter,
    pub _registers: Registers,
}

impl Cpu {
    pub fn new(memory: Memory) -> Cpu {
        return Cpu {
            _counter: ProgramCounter::new(memory),
            _registers: Registers::new(),
        };
    }
    
    pub fn run_next_instruction(&mut self) {
        let instruction = self._counter.read_next_instruction();
        instruction.run(&mut self._registers);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    
    #[test]
    fn test_run_next_instruction_runs_instruction() {
        const ADD_INSTRUCTION: u8 = 0x87;
        const COUNTER: u16 = 0xABCD;
        const INITIAL_A: u8 = 0x12;
        const EXPECTED_A: u8 = INITIAL_A + INITIAL_A;
        
        let mut memory = Memory::new();
        memory.set_byte(COUNTER, ADD_INSTRUCTION);
        
        let mut cpu = Cpu::new(memory);
        cpu._counter.set_counter(COUNTER);
		cpu._registers.a.set(INITIAL_A);
        
        cpu.run_next_instruction();
        
        assert_eq!(as_hex!(cpu._registers.a), as_hex!(EXPECTED_A));
    }
}
