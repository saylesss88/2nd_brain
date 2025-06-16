---
id: unrecoverable_errors
aliases: []
tags: []
---

# Unrecoverable Errors with panic!

There are two ways to cause a panic in practice: by taking an action that causes
our code to panic (like accessing an array past the end) or by explicitly
calling the `panic!` macro.

- By default, these panics will print a failure message, unwind, clean up the
  stack, and quit.

> [!TIP]: Unwinding the stack or Aborting in Response to a Panic: By default,
> when a panic occurs the program starts _unwinding_, which means Rust walks
> back up the stack and calls all the functions that were in the stack trace.
> This cleaning up is a lot of work. Rust allows you to choose the alternative
> of immediately aborting, which ends the program without cleaning up.

- Memory that the program was using will then need to be cleaned up by the
  operating system.

- You can switch from unwinding to aborting by adding the `panic = "abort"` to
  the appropriate `[profile]` sections in your Cargo.toml file:

```rust
[profile.release]
  panic = 'abort
```

```rust
fn main() {
  panic("crash and burn");
}
```
