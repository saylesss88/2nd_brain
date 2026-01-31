# Understanding the `:?` Debug Formatter

The :? is needed because Option<i32> doesn't implement the Display trait, only
Debug. If you want prettier output:

```rust
fn main() {
    let ans = divide(4, 16);

    // Option 1: Print the Debug representation
    println!("ans: {:?}", ans);  // ans: Some(0)

    // Option 2: Handle Some/None explicitly
    match ans {
        Some(value) => println!("ans: {}", value),
        None => println!("Cannot divide by zero!"),
    }

    // Option 3: Use unwrap_or for a default
    println!("ans: {}", ans.unwrap_or(-1));
}
```

Try it with divide(16, 4) to get Some(4) and divide(16, 0) to see None!

We have many options on how to display the result of calling the function.

1. Store in a variable

```rs
let ans = divide(4, 16);
println!("ans: {:?}", ans);
```

2. Use directly in expressions

```rust
// Directly in println!
println!("ans: {:?}", divide(4, 16));

// In calculations
let result = divide(20, 4).unwrap() + 10;

// In conditionals
if divide(10, 0).is_none() {
    println!("Cannot divide by zero!");
}
```

3. Chain methods on the return value

```rust
// Option has lots of methods you can chain
divide(20, 4)
    .map(|x| x * 2)
    .unwrap_or(0);

// Or handle with match directly
match divide(20, 5) {
    Some(val) => println!("Result: {}", val),
    None => println!("Error!"),
}
```

4. Pass directly to another function

```rust
fn double(opt: Option<i32>) -> Option<i32> {
    opt.map(|x| x * 2)
}

let result = double(divide(20, 4));
```

5. Ignore the result (with a warning)

```rust
divide(4, 2);  // Compiles but Rust warns: "unused `Option` that must be used"
```

The idiomatic approach: Use the result directly when possible, store in a
variable only when you need to reuse it or when it improves readability.

```rust
// Not needed - too verbose
let ans = divide(20, 4);
println!("{:?}", ans);

// Better - direct use
println!("{:?}", divide(20, 4));
```

## Using `dbg!` with the divide function

```rs
fn main() {
  let ans = divide(4, 16);
  dbg!(ans);
}
```

**What makes dbg! special**:

1. Shows the file, line number, AND the expression:

```rs
dbg!(divide(20, 4));
// Prints to stderr:
// [src/main.rs:2] divide(20, 4) = Some(5)
```

2. Returns the value, so you can use it inline:

```rs
let result = dbg!(divide(20, 4)).unwrap_or(0);
// Prints the Option, then continues using it
```

3. Can wrap entire expressions:

```rs
let x = dbg!(4 + 2) * 3;
// [src/main.rs:1] 4 + 2 = 6
// x is now 18
```

4. Can debug multiple values at once:

```rs
dbg!(divide(20, 4), divide(10, 0));
// Shows both results as a tuple
```

5. Just prints location with no arguments:

```rs
if some_condition {
  dbg!();    // Shows: [src/main.rs:5]
}
```

> `dbg!` prints to **stderr**, not stdout, so it won't interfere with your
> program's actual output. Perfect for debugging.
