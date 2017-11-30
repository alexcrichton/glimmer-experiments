#![feature(allocator_api)]

use std::heap::{Alloc, Layout, Heap};
use std::mem;

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
        if len < self.stack.len() {
            return Ok(())
        }
        // TODO: Need a way to grow a vector without inducing a panic in
        // libstd, right now just do everything manually.
        unsafe {
            let size = (len + 1).checked_mul(mem::size_of::<u64>()).ok_or(())?;
            let align = mem::align_of::<u64>();
            let layout = Layout::from_size_align(size, align).ok_or(())?;
            let ptr = Heap.alloc(layout).unwrap_or_else(|e| Heap.oom(e));
            let ptr = ptr as *mut u64;
            let mut v = Vec::from_raw_parts(ptr, len + 1, len + 1);
            {
                let mut it = v.iter_mut();
                for (element, slot) in self.stack.iter().zip(&mut it) {
                    *slot = *element;
                }
                for slot in it {
                    *slot = 0;
                }
            }
            self.stack = v;
        }
        Ok(())
    }
}

impl Clone for Stack {
    fn clone(&self) -> Stack {
        // TODO: Right now `Vec::clone` contains a panic in the object code in
        // the standard library that LLVM doesn't optimize away. The panic
        // is related to `Vec::with_capacity` panicking if the capacity is
        // too big. Clearly it's not too big as we've already got a `Vec`, so
        // the standard library needs to be modified to avoid referencing a
        // panic when cloning a vector.
        unsafe {
            let size = self.stack.len() * mem::size_of::<u64>();
            let align = mem::align_of::<u64>();
            let layout = Layout::from_size_align_unchecked(size, align);
            let ptr = Heap.alloc(layout).unwrap_or_else(|e| Heap.oom(e));
            let ptr = ptr as *mut u64;
            let mut v = Vec::from_raw_parts(ptr, 0, self.stack.len());
            v.extend_from_slice(&self.stack[..]);
            Stack { stack: v }
        }
    }
}
