const fs = require('fs');
const process = require('process');
const buffer = fs.readFileSync(process.argv[2]);

Error.stackTraceLimit = 20;

let m = new WebAssembly.Module(buffer);
let instance = new WebAssembly.Instance(m, {
  env: {
    dlog: function(a) {
      console.log('dlog ', a);
    },
  }
});

function assert_eq(a, b) {
  if (a !== b) {
    console.trace(`a != b; a=${a}; b=${b}`);
    process.exit(1);
  }
}

let stack = instance.exports.stack_new();
assert_eq(instance.exports.stack_read(stack, 0), 0);
// assert_eq(instance.exports.stack_len(stack), 0);
assert_eq(instance.exports.stack_write(stack, 4, 8), 1);
// assert_eq(instance.exports.stack_len(stack), 5);
assert_eq(instance.exports.stack_read(stack, 4), 8);

assert_eq(instance.exports.stack_write(stack, 4, -8), 1);
// assert_eq(instance.exports.stack_len(stack), 5);
assert_eq(instance.exports.stack_read(stack, 4), -8);

assert_eq(instance.exports.stack_write(stack, 3, -2), 1);
// assert_eq(instance.exports.stack_len(stack), 5);
assert_eq(instance.exports.stack_read(stack, 4), -8);
assert_eq(instance.exports.stack_read(stack, 3), -2);

let stack2 = instance.exports.stack_clone(stack);
assert_eq(instance.exports.stack_read(stack2, 4), -8);
assert_eq(instance.exports.stack_read(stack2, 3), -2);
instance.exports.stack_free(stack2);

assert_eq(instance.exports.stack_read(stack, 4), -8);
assert_eq(instance.exports.stack_read(stack, 3), -2);
assert_eq(instance.exports.stack_reset(stack));
// assert_eq(instance.exports.stack_len(stack), 0);
assert_eq(instance.exports.stack_read(stack, 4), 0);
assert_eq(instance.exports.stack_read(stack, 3), 0);
