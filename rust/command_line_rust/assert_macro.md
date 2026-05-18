# assert macro

Asserts that a boolean expression is true at runtime.

This will invoke the `panic!` macro if the provided expression cannot be
evaluated to `true` at runtime.

## Uses

Assertions are always checked in both debug and release builds, and can't be
disabled.

Unsafe code may rely on `assert!` to enforce run-time invariants that, if
violated could lead to unsafety.

Also, for testing and enforcing runtime invariants in safe code (whose violation
cannot result in unsafety).

### Custom Messages

This macro has a second form, where a custom panic message can be provided with
or without arguments for formatting. See
[std::fmt](https://doc.rust-lang.org/std/fmt/index.html) for syntax for this
form. Expressions used as format arguments will only be evaluated if the
assertion fails.

### Examples

```rust
// the panic message for these assertions is the stringified value of the
// expression given.
assert!(true);

fn some_computation() -> bool {
    // Some expensive computation here
    true
}

assert!(some_computation());

// assert with a custom message
let x = true;
assert!(x, "x wasn't true!");

let a = 3; let b = 27;
assert!(a + b == 30, "a = {}, b = {}", a, b);
```
