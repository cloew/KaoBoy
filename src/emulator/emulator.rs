use super::Memory;
use super::super::cpu::cpu::Cpu;
use crate::rc_refcell;

use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Emulator {
    pub _cpu: Cpu,
    pub _memory: Rc<RefCell<Memory>>,
}

impl Emulator {
    pub fn new() -> Emulator {
        let memory = rc_refcell!(Memory::new());
        return Emulator {
            _cpu: Cpu::new(memory.clone()),
            _memory: memory.clone(),
        };
    }
    
    pub fn bootstrap(&mut self) {
        let bootstrap_data = fs::read("emulator/bootstrap.bin").expect("Unable to read bootstrap file");
        self._memory.borrow_mut().bootstrap(bootstrap_data);
    }
    
    pub fn run(&mut self) {
        loop {
            self._cpu.run_next_instruction();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bootstrap_sets_bytes() {
        const BOOTSTRAP_BYTES: [u8; 10] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x16, 0x17, 0x18, 0x1A, 0x1B,
        ];
        let mut emulator = Emulator::new();
        
        emulator.bootstrap(&BOOTSTRAP_BYTES);
    }
}
