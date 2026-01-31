# Arithmetic

```rs
fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn main() {
    println!("result: {}", interproduct(120, 100, 248));
}
```

The above `return` statement is unnecessary, you can remove it but a unit type
will be returned `()` unless you remove the trailing semi-colon.

- Expression (no semicolon) -> evaluates to a value that can be returned

- Statement (with semicolon) -> performs an action, returns `()` (unit type)

```rs
fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    a * b + b * c + c * a
}

fn main() {
    println!("result: {}", interproduct(120, 100, 248));
}
```

## Why this Matters

This is idiomatic Rust style. The last expression without a semicolon is
implicitly returned. You typically only use explicit return for:

1. Early returns:

```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;  // Early exit
    }
    Some(a / b)  // Implicit return at end
}
```

2. Clarity in complex functions:

```rust
fn complex_logic(x: i32) -> i32 {
    if x < 0 {
        return -1;
    }
    // ... lots of code ...
    x * 2  // Implicit return makes it clear this is the result
}
```
