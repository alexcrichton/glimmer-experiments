#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(feature = "crazy")] {
        extern crate crazy;
        use crazy::Stack;
    } else if #[cfg(feature = "hand-optimized")] {
        extern crate hand_optimized_vec;
        use hand_optimized_vec::Stack;
    } else {
        extern crate naive;
        use naive::Stack;
    }
}

#[no_mangle]
pub extern fn stack_new() -> usize {
    Stack::new().into_usize()
}

#[no_mangle]
pub unsafe extern fn stack_free(stack: usize) {
    Stack::free_usize(stack);
}

#[no_mangle]
pub unsafe extern fn stack_clone(stack: usize) -> usize {
    Stack::with_stack(stack, |s| s.clone()).into_usize()
}

//   sliceFrom(start: u32): Stack {
//     return new Stack(this.vec.slice(start));
//   }
//
//   slice(start: u32, end: i32): Stack {
//     return new Stack(this.vec.slice(start, end));
//   }

#[no_mangle]
pub unsafe extern fn stack_copy(stack: usize, from: u32, to: u32) -> u32 {
    Stack::with_stack(stack, |s| s.copy(from, to)).is_ok() as u32
}

#[no_mangle]
pub unsafe extern fn stack_write_raw(stack: usize, at: u32, val: u64) -> u32 {
    Stack::with_stack(stack, |s| s.write(at, val)).is_ok() as u32
}

#[no_mangle]
pub unsafe extern fn stack_write(stack: usize, at: u32, val: i32) -> u32 {
    Stack::with_stack(stack, |s| s.write(at, encode(val))).is_ok() as u32
}

const NUMBER: u64 = 0b000;
const NEGATIVE: u64 = 0b100;
const MASK: u64 = 0b111;

fn encode(val: i32) -> u64 {
    let (val, flags) = if val < 0 {
        ((-val) as u64, NEGATIVE)
    } else {
        (val as u64, NUMBER)
    };
    (val << 3) | flags
}

#[no_mangle]
pub unsafe extern fn stack_read_raw(stack: usize, at: u32) -> u64 {
    Stack::with_stack(stack, |s| s.read(at)).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern fn stack_read(stack: usize, at: u32) -> i32 {
    decode(Stack::with_stack(stack, |s| s.read(at)).unwrap_or(0))
}

fn decode(val: u64) -> i32 {
    let payload = val >> 3;
    match val & MASK {
        NUMBER => payload as i32,
        NEGATIVE => -(payload as i32),
        _ => 0, // TODO: panic?
    }
}

#[no_mangle]
pub unsafe extern fn stack_reset(stack: usize) {
    Stack::with_stack(stack, |s| s.reset())
}

#[no_mangle]
pub unsafe extern fn stack_len(stack: usize) -> u32 {
    Stack::with_stack(stack, |s| s.len())
}
