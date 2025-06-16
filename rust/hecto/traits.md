---
id: traits
aliases: []
tags: []
---

# Traits

```rust
println!("x is {:?}",x);
```

- This only works if Rust knows how to create a `Debug` representation of `x`.
  Rust knows how to do that for easy data types like `u16`, but not for `struct`s

- So Rust needs a way to tell us: Hey, in order to print this debug thing, tell
  me how! And we need a way to tell Rust: _this is how you do it!_.

- What we're looking for are called `traits`. A trait is just that: A statement
  that says "You need to implement a couple of things for this to work".

If we want our new test struct, `Hecto`, to show up as a debug string do this:

```rust
struct Hecto;
fn main() {
  let x = Hecto;
  println!("{:?}",x);
}
```

- This doesn't compile, and the compiler tells us why:

```text
= help: the trait 'Debug' is not implemented for `Hecto`
```

- "The trait is not implemented" means something like "we have to code something
  so that this works".

The Docs tell us what this trait looks like:

```rust
pub trait Debug {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

- This says: This is a trait called `Debug`, and in order to implement this
  trait, you need to implement a function named `fmt` with that specific
  signature.

The below code would also fail, if you tried something else:

```rust
println!("x is {}", x);
```

- This fails because Rust wants you to implement the `Display` trait.

```rust
pub trait Display {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

Since the point here is not how the formatter works, but: How do we tell Rust
that we want to implement this method?

The answer is that we are implementing the function in a special `impl` block,
which looks like this:

```rust
impl Debug for Hecto {
  // Implementation goes here
}
```

- This meand: "This implementation block implements the `Debug` trait for the
  data type `Hecto`"

```rust
use std::fmt::{Debug, Result, Formatter};
struct Hecto;
impl Debug for Hecto {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Hecto is awesome.")
  }
}
fn main() {
  let x = Hecto;
  println!("{:?}",x);
}
```

- What we do here is say: whatever `Hecto` is, whatever it contains, the only
  debug information you'll ever need is "Hecto is awesome."
