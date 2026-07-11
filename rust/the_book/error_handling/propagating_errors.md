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

This is manual propagation, propagating an error out of a function.

- The return type of the function is `Result<String, io::Error>` This means the
  function is returning a value of type `Result<T, E>`, where the generic
  parameter `T` is filled with a concrete `String` and the generic type `E` is
  filled with `io::Error`.

- If this function succeeds, the code that calls this function will receive an
  `Ok` value that holds a `String` the `username` that the function reads from
  the file.

- If the function fails, the calling code will receive an `Err` value that holds
  an instance of `io::Error`

Example: The calling code:

```rs
fn main() {
    let username = match read_username_from_file() {
        Ok(un) => un,
        Err(e) => e.to_string(),
    };

    print!("Username: {username:?}");
}
```

## Propagating errors with ?

The `?` in `read_username_from_file` vs. manual `match`: this is about
propagating an error out of a function.

Manual:

```rs
let mut username_file = match username_file_result {
  Ok(file) => file,
  Err(e) => return Err(e),
};
```

`?` is sugar for exactly that pattern. If `Ok`, unwrap and keep going; if `Err`,
return it immediately from the enclosing function. We could rewrite it as:

```rs
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username.trim().to_string())
}
```

If the value of the `Result` is an `Ok`, the value inside the `Ok` will get
returned from this expression, and the program will continue. If the value is an
`Err`, the `Err` will be returned from the whole function as if we had used the
`return` keyword so that the error value gets propagated to the calling code.

There is a difference between what the `match` expression and what the `?`
operator does: Error values that have the `?` operator called on them go through
the `from` function, defined in the `From` trait in stdlib, which is used to
convert values from one type into another. When the `?` operator calls the
`from` function, the error type received is converted into the error type
defined in the return type of the current function. This is useful when a
function returns one error type to represent all the ways a function might fail,
even if parts might fail for many different reasons.

Same behavior, way less boilerplate.

The calling code would be the same, they still need to handle the possibility of
failure when calling in the same way. You can also use `map` and
`unwrap_or_else` to reduce the case analysis with `match`.

```rs
fn main() {
    let username = read_username_from_file()
        .map(|s| format!("got: {s}"))
        .unwrap_or_else(|e| format!("error: {e}"));
    print!("Username: {username:?}");
}
```

```txt
# hello.txt
ferris
```

Output:

```text
Username: "got: ferris"
```

Even more concise version:

```rs
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

And even more concise:

```rs
use std::{io,fs};

fn read_username_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}
```

Reading a file into a string is a fairly common operation, so the standard
library provides the convenient `fs::read_to_string` function that opens the
file, creates a new `String`, reads the contents of the file, puts the contents
into that `String`, and returns it. Of course, using `fs::read_to_string`
doesn’t give us the opportunity to explain all the error handling, so we did it
the longer way first.

```rs
fn read_username_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt").map(|s| s.trim().to_string())
}
```

`map` transforms the `Ok` value, and leaves the `Err` alone.

## Using `?` on Option types

```rs
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

This returns `Option<char>` because it's possible that there is a char there,
but it's also possible that there isn't. This code takes the `text` string slice
argument and calles the `lines` method on it, which returns an iterator over the
lines in the string. Because this function wants to examine the first line, it
calls `next` on the iterator to get the first value from the iterator. If `text`
is empty, this call to `next` will return `None`, in which case we use `?` to
stop and return `None` from `last_char_of_first_line`. If `text` isn't empty,
`next` will return a `Some` value containing a string slice of the first line in
`text`.
