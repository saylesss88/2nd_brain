## Reading Signatures

- [Improving Error handling](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html)

```rust
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    println!("Searching for {query}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n {contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
```

We’re still collecting the command line arguments into a vector, but instead of assigning the argument value at index 1 to the variable query and the argument value at index 2 to the variable file_path within the main function, we pass the whole vector to the parse_config function. The parse_config function then holds the logic that determines which argument goes in which variable and passes the values back to main. We still create the query and file_path variables in main, but main no longer has the responsibility of determining how the command line arguments and variables correspond.

### The Signature

1. The Square Brackets `[T]`

In Rust, `[T]` denotes a Slice. A slice is a view into a contiguous sequence of
elements of type `T`.

- If you see `[i32]`, it's a sequence of integers.
- If you see `[String]`, it's a sequence of `String` objects.

2. The Borrow `&`

You almost never see a slice on its own like (`args: [String]`) because slices
have a "dynamically sized" nature. The compiler doesn't know how big they are at
compile time. Therefore, they must sit behind a pointer.

- `&[String]` is a **string slice reference**.

3. Why "Vector"?

While the code technically asks for a slice, in practice, this is how you pass a
`Vec<String>` to a function.

- A `Vec<String>` owns the data.
- A `&[String]` is a borrowed view of that data.

Rust allows a `Vec` to be treated as a slice automatically through a feature
called deref coercion. So, if you have a `Vec<String>`, you pass it to this
function as `&args`.
