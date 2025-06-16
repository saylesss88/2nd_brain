---
id: The Borrow Checker
aliases: [lifetimes]
---

# The Borrow Checker

The Rust compiler has a _borrow checker_ that compares scopes to determine whether all borrows are valid.

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+
```

- This annotates the lifetimes of `r` and `x` named `'a` and `'b`

```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+
```

Output: `r: 5`

- Here, `x` has the lifetime `'b`, which in this case is larger than `'a` This means `r` can reference `x` because Rust knows that the reference in `r` will always be valid while `x` is valid.

[[generic_lifetimes_in_functions.md]]
