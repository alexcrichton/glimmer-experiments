use std::iter;

#[derive(Clone)]
pub struct Stack {
    stack: Vec<u64>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { stack: Vec::new() }
    }

    pub fn into_usize(self) -> usize {
        Box::into_raw(Box::new(self)) as usize
    }

    pub unsafe fn free_usize(stack: usize) {
        Box::from_raw(stack as *mut Stack);
    }

    pub unsafe fn with_stack<F, R>(stack: usize, f: F) -> R
        where F: FnOnce(&mut Stack) -> R
    {
        let stack = stack as *mut Stack;
        f(&mut *stack)
    }

    pub fn copy(&mut self, from: u32, to: u32) -> Result<(), ()> {
        let val = match self.stack.get(from as usize) {
            Some(val) => *val,
            None => return Err(()),
        };
        let dst = match self.stack.get_mut(to as usize) {
            Some(val) => val,
            None => return Err(()),
        };
        *dst = val;
        Ok(())
    }

    pub fn write(&mut self, at: u32, val: u64) -> Result<(), ()> {
        let at = at as usize;
        self.grow_to(at)?;
        match self.stack.get_mut(at) {
            Some(slot) => *slot = val,
            None => return Err(()),
        }
        Ok(())
    }

    pub fn read(&self, at: u32) -> Option<u64> {
        self.stack.get(at as usize).cloned()
    }

    pub fn len(&self) -> u32 {
        self.stack.len() as u32
    }

    pub fn reset(&mut self) {
        self.stack.truncate(0);
    }

    fn grow_to(&mut self, len: usize) -> Result<(), ()> {
        let stack_len = self.stack.len();
        if len < stack_len {
            return Ok(())
        }
        self.stack.extend(iter::repeat(0).take(len - stack_len + 1));
        Ok(())
    }
}
