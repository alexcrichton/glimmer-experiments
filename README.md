# Rust + Glimmer experiments

Currently this repository takes [this abstraction][stack] and ports it directly
to Rust. The interface here is a cdylib at `src/lib.rs` and has a few
implementations currently for assessing size:

[stack]: https://github.com/glimmerjs/glimmer-vm/blob/3e4f6a3e491483a297e7d65148478f931f074809/packages/%40glimmer/low-level/lib/asm/stack.ts

* `naive` - this is an implementation built directly on `Vec` in the standard
  library. 100% safe, easy to read, easy to write.
* `hand-optimized-vec` - this is also using `Vec` but hand-optimizes a few
  operations that don't optimize well in the standard library. For example
  `Clone` and `push` are optimized to have better codegen and generate less
  code.
* `crazy` - this is a 100% *unsafe* implementation that uses the raw wasm
  instructions to allocate memory. It's intended to be the "limit" but is hard
  to read, write, and modify. Wouldn't recommend using.

The `foo.sh` script will compile everything (using cargo features on the main
crate) and run the `test.js` script to ensure at least a smoke test passes. The
size for each of these implementations is:

| implementation | gz size in bytes |
|---------------|------------------|
| naive   |   15551 |
| hand-optimzied | 3343 |
| crazy | 645 |

