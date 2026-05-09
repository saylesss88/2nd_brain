# Better Error Handling

```rs
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n {contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        if args.len() < 3 {
            panic!("not enough arguments")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Self { query, file_path }
    }
}
```

## Returning Result

Because many programmers expect `new` functions to never fail, we'll change the
name to `build`.

```rs
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n {contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
```

- `unwrap_or_else` is defined on `Result<T, E>` by the standard library. This
  allows us to define some custom, non-`panic!` error handling. If the `Result`
  is an `Ok` value, this method's behavior is similar to `unwrap`: It returns
  the inner value that `Ok` is wrapping.
  - If it's an `Err` value, this method calls the code in the closure, which is
    an anonymous function we define and pass as an argument to `unwrap_or_else`.
    Which in this case is `"not enough arguments"`


