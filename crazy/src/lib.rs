#![feature(link_llvm_intrinsics)]

use std::mem;
use std::ptr;

const PAGE_SIZE: usize = 64 * 1024;

pub struct Stack {
    head: *mut Node,
}

struct Node {
    next: *mut Node,
    data: [u64; PAGE_SIZE / 8 - 1],
}

impl Stack {
    pub fn new() -> Stack {
        Stack { head: alloc() }
    }

    pub fn into_usize(self) -> usize {
        self.head as usize
    }

    pub unsafe fn free_usize(stack: usize) {
        // sadface :(
        drop(stack);
    }

    pub unsafe fn with_stack<F, R>(stack: usize, f: F) -> R
        where F: FnOnce(&mut Stack) -> R
    {
        f(&mut Stack { head: stack as *mut Node })
    }

    pub fn copy(&mut self, from: u32, to: u32) -> Result<(), ()> {
        let val = match self.read(from) {
            Some(val) => val,
            None => return Err(()),
        };
        self.write(to, val)
    }

    pub fn write(&mut self, at: u32, val: u64) -> Result<(), ()> {
        unsafe {
            let mut at = at as usize;
            let mut cur = self.head;
            while at >= (*cur).data.len() {
                at -= (*cur).data.len();
                if (*cur).next.is_null() {
                    (*cur).next = alloc();
                }
                cur = (*cur).next;
            }
            (*cur).data[at] = val;
            Ok(())
        }
    }

    pub fn read(&self, at: u32) -> Option<u64> {
        unsafe {
            let mut at = at as usize;
            let mut cur = self.head;
            while at >= (*cur).data.len() {
                at -= (*cur).data.len();
                cur = (*cur).next;
                if cur.is_null() {
                    return None
                }
            }
            Some((*cur).data[at])
        }
    }

    pub fn len(&self) -> u32 {
        0 // uhh ...
    }

    pub fn reset(&mut self) {
        unsafe {
            let mut cur = self.head;
            while !cur.is_null() {
                for slot in (*cur).data.iter_mut() {
                    *slot = 0;
                }
                cur = (*cur).next;
            }
        }
    }
}

impl Clone for Stack {
    fn clone(&self) -> Stack {
        unsafe {
            let ret = Stack { head: alloc() };
            let mut a = ret.head;
            let mut b = self.head;
            loop {
                (*a).data = (*b).data;
                if (*b).next.is_null() {
                    break
                }
                b = (*b).next;
                (*a).next = alloc();
                a = (*a).next;
            }
            return ret
        }
    }
}

fn alloc() -> *mut Node {
    if mem::size_of::<Node>() > PAGE_SIZE {
        return ptr::null_mut()
    }

    extern {
        #[link_name = "llvm.wasm.current.memory.i32"]
        fn current_memory() -> u32;

        // TODO: this intrinsic actually returns the previous limit, but LLVM
        // doesn't expose that right now. When we upgrade LLVM stop using
        // `current_memory` above.
        #[link_name = "llvm.wasm.grow.memory.i32"]
        fn grow_memory(pages: u32);
    }

    unsafe {
        let cur = current_memory() as usize;
        grow_memory(1);
        (cur * PAGE_SIZE) as *mut Node
    }
}
