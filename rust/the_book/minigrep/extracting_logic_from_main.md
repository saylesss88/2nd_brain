# Extracting Logic from `main`

We'll extract a function named `run` that will hold all the logic currently in
the `main` function that isn't involved with setting up configuration or
handling errors.

```rust
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
```

- `run` still returns a unit type in the `Ok` case as seen in the return value:
  `Result<(), Box<dyn Error>`.

- `Box<dyn Error>` means that the function will return a type that implements
  the `Error` trait. This gives us flexibility to return error values that may
  be of different types in different error cases. `dyn` = dynamic.

- We removed `expect` in favor of the `?` operator. Rather than `panic!` on an
  error, `?` will return the error value from the current function for the
  caller to handle.
  - Using `Ok(())` is the idiomatic way to indicate that we're calling `run` for
    its side effects only; it doesn't return a value we need.
  - We use `if let` rather than `unwrap_or_else` to check whether run returns an
    `Err` value and to call `process::exit(1)` if it does. The run function
    doesn’t return a value that we want to `unwrap `in the same way that
    `Config::build` returns the Config instance. Because run returns () in the
    success case, we only care about detecting an error, so we don’t need
    `unwrap_or_else` to return the unwrapped value, which would only be `()`.

Since `run` now returns a `Result` type, we need to change how we call it:

```rust
if let Err(e) = run(config) {
  println!("Application error: {e}");
  process::exit(1);
}
```
