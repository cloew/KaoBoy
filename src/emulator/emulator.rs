use super::Memory;
use super::super::cpu::cpu::Cpu;
use crate::rc_refcell;

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
    
    pub fn bootstrap(&mut self, bootstrap_data: &[u8]) {
        self._memory.borrow_mut().bootstrap(bootstrap_data);
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
