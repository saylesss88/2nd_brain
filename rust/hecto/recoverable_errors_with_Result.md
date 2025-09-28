---
id: recoverable_errors_with_Result
aliases: []
tags: []
---

# Recoverable Errors with Result

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

- The `T` and `E` are generic type parameters.

    - `T` represents the type of the value that will be returned in a success case
      within the `Ok` variant.

    - `E` represents the type of the error that will be returned in a failure case
      within the `Err` variant.

- Because `Result` has these generic type parameters, it gives it flexibility to
  be used in many different situations where the succes value and error value we
  want to return may differ.

This function could fail so we'll return a `Result` value:

```rust
use std::fs::File;

fn main() {
  let greeting_file_result = File::open("hello.txt");

  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {error:?}"),
  };
}
```

- The return type of `File::open` is a `Result<T, E>`

    - The generic parameter `T` has been filled in by the implementation of
      `File::open` with the type of the success value, `std::fs::File`, which is a
      file handle.

    - The type `E` used in the error value is `std::io::Error`.

- The `File::open` function needs a way to tell us whether it succeeds or fails
  and at the same time give use either the file handle or error information.
  This information is exactly what the `Result` enum conveys.
