---
id: propagating_errors
aliases: []
tags: []
---

# Propagating Errors

When a function's implementation calls something that might fail, instead of
handling the error within the function itself you can return the error to the
calling code so that it can decide what to do. This is called _propagating_ the
error and gives more control to the calling code.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
  let username_file_result = File::open("hello.txt");

  let mut username_file = match username_file_result {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut username = String::new();

  match username_file.read_to_string(&mut username) {
    Ok(_) => Ok(username),
    Err(e) => Err(e),
  }
}
```

- The return type of the function is `Result<String, io::Error>` This means the
  function is returning a value of type `Result<T, E>`, where the generic
  parameter `T` is filled with a concrete `String` and the generic type `E` is
  filled with `io::Error`.

- If this function succeeds, the code that calls this function will receive an
  `Ok` value that holds a `String` the `username` that the function reads from the
  file.

- If the function fails, the calling code will receive an `Err` value that holds
  an instance of `io::Error`
