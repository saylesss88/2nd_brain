# The Result Enum

The purpose of `Result` types is to encode error-handling information.

The `Result` enum is defined as having two variants, `Ok` and `Err`:

```rs
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

The `T` and `E` are generic parameters. `T` represents the type of the value
that will be returned in a success case within the `Ok` variant, and `E`
represents the type of error that will be returned in a failure case within the
`Err` variant.

Values of the `Result` type, have methods defined on them. An instance of
`Result` has an `expect` method that you can call. If the instance of `Result`
is an `Err` value, `expect` will cause the program to crash and display the
message that you passed as an argument to `expect`. If the instance of `Result`
is an `Ok` value, `expect` will take the return value that `Ok` is holding and
return just that value to you so you can use it.

The following function returns a `Result` value because it could fail:

```rs
use std::fs::File;

fn main() {
  let greeting_file_result = File::open("hello.txt");
}
```

The return type of `File::open` is a `Result<T, E>`.
