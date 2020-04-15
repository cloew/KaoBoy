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
    
    pub fn get_pointer(&self) -> u16 {
        return self._pointer;
    }
    
    pub fn set_pointer(&mut self, new_pointer: u16) {
        self._pointer = new_pointer;
    }
    
    pub fn pop(&mut self) -> u16 {
        let value = self._memory.borrow_mut().read_short(self._pointer);
        self._pointer += 2;
        return value;
    }
    
    pub fn push(&mut self, value: u16) {
        self._pointer -= 2;
        self._memory.borrow_mut().write_short(self._pointer, value);
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
        
        assert_eq!(stack.get_pointer(), NEW_STACK_POINTER);
    }
    
    #[test]
    fn test_pop_reads_memory_at_stack_pointer() {
        const STACK_POINTER: u16 = 0xABCD;
        const VALUE: u16 = 0xCAB0;
        let mut stack = build_stack();
        stack.set_pointer(STACK_POINTER);
        stack._memory.borrow_mut().write_short(STACK_POINTER, VALUE);
        
        let result = stack.pop();
        
        assert_eq!(result, VALUE);
    }
    
    #[test]
    fn test_pop_updates_stack_pointer() {
        const STACK_POINTER: u16 = 0xABCD;
        const NEW_STACK_POINTER: u16 = STACK_POINTER + 2;
        let mut stack = build_stack();
        stack.set_pointer(STACK_POINTER);
        
        stack.pop();
        
        assert_eq!(stack.get_pointer(), NEW_STACK_POINTER);
    }
    
    #[test]
    fn test_push_writes_memory_at_stack_pointer() {
        const STACK_POINTER: u16 = 0xABCD;
        const VALUE: u16 = 0xCAB0;
        let mut stack = build_stack();
        stack.set_pointer(STACK_POINTER);
        
        stack.push(VALUE);
        
        assert_eq!(stack._memory.borrow().read_short(STACK_POINTER-2), VALUE);
    }
    
    #[test]
    fn test_push_updates_stack_pointer() {
        const STACK_POINTER: u16 = 0xABCD;
        const NEW_STACK_POINTER: u16 = STACK_POINTER - 2;
        const VALUE: u16 = 0xCAB0;
        let mut stack = build_stack();
        stack.set_pointer(STACK_POINTER);
        
        stack.push(VALUE);
        
        assert_eq!(stack.get_pointer(), NEW_STACK_POINTER);
    }
}