# Attributes

- `#[derive(Parser)]` <- These are called Attributes, they are metadata that
  tell the compiler (or a lib like `clap`) to do something special with it. Kind
  of like stickers on a box with special instructions "Handle with care".

1. **What do they do**?

They don't change the logic of your code directly (like an if statement does).
Instead, they typically:

- Generate code for you: `#[derive(Debug)]` writes the code to make
  `println!("{:?}", my_struct`) work.

- Change behavior: `#[test]` tells Rust "this function is a test, run it when I
  type `cargo test`."

- Configure things: `#[arg(short)]` tells Clap "use `-s` for this argument."

2. **The Syntax**

- `#[ ... ]` (Outer Attribute): Applies to the thing **immediately following
  it** (like a struct or function).

```rust
#[derive(Parser)] // <-- Applies to the struct below
struct Cli { ... }
```

- `#![ ... ]` (Inner Attribute, note the !): Applies to the thing it is inside
  of (usually the whole file).

```rust
// main.rs top line
#![allow(unused_variables)] // <-- Applies to this whole file
```

3. Common Attributes You'll See

- `#[derive(...)]`: "Please write the code for these traits automatically."

- `#[cfg(test)]`: "Only compile this block when testing."

- `#[allow(dead_code)]`: "Don't warn me if I don't use this function."

In your clap example:

```rust
#[derive(Parser)] // "Hey compiler, write the Cli::parse() function for this struct automatically!"
struct Cli {
    #[arg(short)] // "Hey Clap, make this field accessible via a short flag like -p"
    pattern: String,
}
```

So whenever you see #[...], just think: "This is a special instruction for the
compiler about the next block of code."
