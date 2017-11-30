set -ex

doit() {

  cargo +nightly build --release --target wasm32-unknown-unknown "$@"

  dir=target/wasm32-unknown-unknown/release
  wasm-gc $dir/stack.wasm $dir/stack.wasm.gc
  wasm-opt -Os $dir/stack.wasm.gc -o $dir/stack.wasm.gc.opt
  gzip -9 -fk $dir/stack.wasm.gc.opt
  ls -alh $dir/stack.wasm*

  node test.js $dir/stack.wasm.gc.opt
}

doit
doit --features hand-optimized
doit --features crazy
