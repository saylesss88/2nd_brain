# Parsing CLI arguments with Clap

**Two main ways to use `clap`**

1. Derive API (Recommended): You define a struct, add `#[derive(Parser)]`, and
   `clap` generates the parsing logic for you.

2. Builder API: You manually build the parser in code using function calls like
   `Command::new("prog").arg(...)`. This doesn't require a struct at all, but is
   more verbose.

---

- [Command Line Applications in Rust (cli-args)](https://rust-cli.github.io/book/tutorial/cli-args.html)

Add `clap` as a dependency in your `Cargo.toml`:
`clap = { version = "4.5.54", features = ["derive"] }`

Add `use clap::Parser;` to `main.rs` and add `#[derive(Parser)]` right above a
struct.

So with `clap` our program can change from this:

```rs
struct Cli {
  pattern: String,
  path: std::path::Pathbuf,
}
fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = Cli {
        pattern,
        path: std::path::PathBuf::from(path),
    };

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
```

To this:

```rs
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
}
```

The key insight here is how `clap` transforms manual, fragile work into
declarative, robust code.

Breakdown

1. Manual Indexing vs. Named Fields

**Before**:

```rs
let pattern = std::env::args().nth(1).expect("no pattern given");
```

You rely on position (`nth(1)`). If the user types `my_program path pattern`
instead of `my_program pattern path`, the code breaks or does the wrong thing
silently.

**After**:

```rs
#[derive(Parser)]
struct Cli {
  pattern: String,
  path: std::path::Pathbuf,
}
```

You rely on names and types. Clap figures out "pattern" goes into pattern and
"path" goes into path. It handles the parsing logic for you.

2. "Panic" vs. Helpful Errors

**Before**

If you forget an argument:

```text
thread 'main' panicked at 'no pattern given', src/main.rs:6:43
```

Your program crashes with an ugly developer error.

**After**

If you forget an argument:

```text
error: the following required arguments were not provided:
  <PATTERN>
  <PATH>

Usage: my_program <PATTERN> <PATH>

For more information, try '--help'.
```

3. Type Conversion is Automated Before:

```rust
path: std::path::PathBuf::from(path),
```

You manually convert the string from `env::args()` into a `PathBuf`.

After:

```rust
path: std::path::PathBuf,
```

Clap sees the type is `std::path::PathBuf` and automatically converts the input
string for you. If it fails (e.g., trying to parse "hello" into a `u32`), Clap
generates an error message for the user automatically.

4. Free Documentation

**After**:

Because you added `///` comments:

```rust
/// The pattern to look for
pattern: String,
```

Clap automatically generates a `--help` page:

```text
Arguments:
  <PATTERN>  The pattern to look for
  <PATH>     The path to the file to read
```

In the manual version, you have 0 documentation unless you write a massive
`println!` block yourself.

The "Hammer Home" Point: Clap isn't just "parsing arguments." It turns your
struct definition into a full user interface (parsing, validation, help text,
error handling) for free. You declare what you want, and Clap handles how to get
it from the user.
