# Recoverable Errors with Result

The `Result` enum is defined as having two variants, `Ok`, and `Err`:

```rs
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

The `T` and `E` are generic type parameters where `T` represents the type of the
value that will be returned in a success case within the `Ok` variant, and `E`
represents the type of the error that will be returned in a failure case within
the `Err` variant.

Let's call a function that returns a `Result` value because the function could
fail:

```rs
use std::fs::File;

fn main() {
  let greeting_file_result = File::open("hello.txt");
}
```

The return type of `File::open` is a `Result<T, E>`. The generic parameter `T`
has been filled in by the implementation of `File::open` with the type of the
success value, `std::fs::File`, which is a file handle.

The type of `E` used in the error value is `std::io::Error`. This return type
means the call to `File::open` might succeed and return a file handle that we
can read from or write to. The function call also might fail: if the file didn't
exist, or we don't have permission to access the file. The `File::open` function
needs a way to tell us whether it succeeded or failed and at the same time give
us either the file handle or error information. This is exactly what `Result`
provides.

In the case where `File::open` succeeds, the value in the variable
`greeting_file_result` will be an instance of `Ok` that contains a file handle.
In the case where it fails, the value in greeting_file_result will be an
instance of `Err` that contains more information about the kind of error that
occurred.

Let's add the code to take different actions depending on the value `File::open`
returns.

```rs
use std::fs::File;

fn main() {
  let greeting_file_result = File::open("hello.txt");

  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {error:?}"),
  };
}
```

## Matching on Different Errors

The code above will `panic!` no matter why `File::open` failed. We want to take
different actions for different failure reasons. If `File::open` failed because
the file doesn't exist, we want to create the file and return the handle of the
new file. If it fails for any other reason, we still want to `panic!`:

```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let greeting_file_result = File::open("hello.txt");

  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {e:?}"),
      },
      _ => {
        panic!("Problem opening the file: {error:?}");
      }
    },
  };
}
```

Alternative to using `match` with `Result<T, E>`

```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
```

## Shortcuts for Panic on Error

The `unwrap` method is a shortcut method implemented just like the `match` expression
above. If the `Result` value is the `Ok` variant, `unwrap` will return the value
inside the `Ok`. If the `Result` is the `Err` varaint, `unwrap` will call the
`panic!` macro for us.

```rs
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

`expect` is the same, but let's you choose the `panic!` message:

```rs
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```
