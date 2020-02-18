use crate::emulator::Memory;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Stack {
    _pointer: u16,
    pub _memory: Rc<RefCell<Memory>>,
}

impl Stack {
    pub fn new(memory: Rc<RefCell<Memory>>) -> Stack {
        return Stack {_pointer: 0, _memory: memory};
    }
    
    pub fn set_pointer(&mut self, new_pointer: u16) {
        self._pointer = new_pointer;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex, rc_refcell};
    
    fn build_stack() -> Stack {
        return Stack::new(rc_refcell!(Memory::new()));
    }
    
    #[test]
    fn test_set_pointer_updates_stack_pointer() {
        const NEW_STACK_POINTER: u16 = 0xABCD;
        let mut stack = build_stack();
        
        stack.set_pointer(NEW_STACK_POINTER);
        
        assert_eq!(stack._pointer, NEW_STACK_POINTER);
    }
}