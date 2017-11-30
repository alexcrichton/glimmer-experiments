#![feature(link_llvm_intrinsics, allocator_api)]

use std::mem;

pub struct Stack {
    head: *mut Node,
}

struct Node {
    next: *mut Node,
    data: [u64; page::PAGE_SIZE / 8 - 1],
}

impl Stack {
    pub fn new() -> Stack {
        Stack { head: page::alloc() as *mut Node }
    }

    pub fn into_usize(self) -> usize {
        self.head as usize
    }

    pub unsafe fn free_usize(stack: usize) {
        drop(Stack { head: stack as *mut Node });
    }

    pub unsafe fn with_stack<F, R>(stack: usize, f: F) -> R
        where F: FnOnce(&mut Stack) -> R
    {
        let mut tmp = Stack { head: stack as *mut Node };
        let ret = f(&mut tmp);
        mem::forget(tmp);
        return ret
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
                    (*cur).next = page::alloc() as *mut Node;
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
            let ret = Stack { head: page::alloc() as *mut Node };
            let mut a = ret.head;
            let mut b = self.head;
            loop {
                (*a).data = (*b).data;
                if (*b).next.is_null() {
                    break
                }
                b = (*b).next;
                (*a).next = page::alloc() as *mut Node;
                a = (*a).next;
            }
            return ret
        }
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe {
            let mut cur = self.head;
            while !cur.is_null() {
                let next = (*cur).next;
                page::free(cur as *mut page::Page);
                cur = next;
            }
        }
    }
}

mod page {
    use std::heap::{Alloc, Heap, AllocErr};

    pub const PAGE_SIZE: usize = 64 * 1024;
    pub type Page = [u8; PAGE_SIZE];

    static mut NEXT_FREE: *mut List = 0 as *mut _;

    struct List {
        next: *mut List,
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

    pub fn alloc() -> *mut Page {
        unsafe {
            if NEXT_FREE.is_null() {
                let cur = current_memory() as usize;
                grow_memory(1);
                if current_memory() as usize == cur {
                    Heap.oom(AllocErr::invalid_input("oom"))
                }
                (cur * PAGE_SIZE) as *mut Page
            } else {
                let ret = NEXT_FREE;
                NEXT_FREE = (*ret).next;
                ret as *mut Page
            }
        }
    }

    pub unsafe fn free(page: *mut Page) {
        let page = page as *mut List;
        (*page).next = NEXT_FREE;
        NEXT_FREE = page;
    }
}
