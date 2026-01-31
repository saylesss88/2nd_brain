## First Implementation of grrs

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
}
```

- `#[derive(Parser)]` <- These are called Attributes, they are metadata that
  tell the compiler (or a lib like `clap`) to do something special with it. Kind
  of like stickers on a box with special instructions "Handle with care".

- `Cli::parse()` parses `std::env::args_os()` (terminal arguments)

- `Cli::parse_from()` parses any iterator of strings you give it.

1. **Opening the file**: Lets open the file we get:

```rs
let content = std::fs::read_to_string(&args.path).expect("could not read file");
```

> Note: See that `.expect` method here? This is a shortcut function that will
> make the program exit immediately when the value (in this case, the input
> file) could not be read. It’s not very pretty, and in the next chapter on
> Nicer error reporting, we will look at how to improve this.

2. Iterate over the lines and print each one that contains our pattern:

```rs
for line in content.lines() {
  if line.contains(&args.pattern) {
    println!("{}", line);
  }
}
```
