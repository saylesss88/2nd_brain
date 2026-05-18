# Writing and Running Integration Tests

```rust
use std::process::Command;

#[test]
fn runs() {
    let mut cmd = Command::new("ls");   // Create a new `Command` to run `ls`
    let res = cmd.output();             // Run the command and capture the output, which will be a Result
    assert!(res.is_ok());               // Verify the result is the `Ok` variant
}
```

Using the basic "Hello World!" example we write a test in `tests/cli.rs`:

```rs
use std::process::Command;

#[test]
fn runs() {
    let mut cmd = Command::new("hello");
    let res = cmd.output();
    assert!(res.is_ok());
}
```

This test fails because `hello` isn't in our PATH. It only exists in
_target/debug/hello_.

List directories in PATH, replacing colons with newlines (`\n`):

```bash
echo $PATH | tr : '\n'
/home/jr/.cargo/bin
/home/jr/.local/bin
/home/jr/bin
/usr/local/bin
/usr/bin
/bin
/usr/local/sbin
/usr/bin/site_perl
/usr/bin/vendor_perl
/usr/bin/core_perl
/usr/lib/rustup/bin
```

We can move the binary to any of these directories and have the test run
successfully. But we don't want to copy our program to test it; we want to test
the program that lives in the current crate.

We'll use:

- [assert_cmd](https://oreil.ly/Lw-gr) to find the program in the crate
  directory.

- [pretty_assertions](https://oreil.ly/VqD62) to use a version of `assert_eq!`
  macro that shows differences between two strings better than the default.

We'll add these as development dependencies in our _Cargo.toml_. Telling Cargo
that we only need these crates for testing and benchmarking.

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"

[dependencies]

[dev-dependencies]
assert_cmd = "2.0.13"
pretty_assertions = "1.4.0"
```

Now we can update _tests/cli.rs_ to use `assert_cmd::Command` instead of
`std::process::Command`:

```rust
use assert_cmd::Command;

#[test]
fn runs() {
  let mut cmd = Command::cargo_bin("hello").unwrap();
  cmd.assert().success();
}
```

- Create a `Command` to run `hello` in the current crate. This returns a
  `Result`, and the code calls `Result::unwrap` because the binary should be
  found.

- Use `Assert::success` to ensure the command succeeded.

```bash
cargo test
running 1 test
test runs ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Understanding Exit Codes

The Portable Operating System Interface (POSIX) standards dictate that the
standard exit code is 0 to indicate success (think zero errors) and any number
from 1 to 255 otherwise.

- [bash shell Special Parameters](https://www.thegeekstuff.com/2010/05/bash-shell-special-parameters/)

- `$?` Gives the exit status of the most recently executed command.

```bash
true
echo $?
0

false
echo $?
1
```

All programs we write are expected to return zero when they terminate normally
and a nonzero value when there is an error.

## Rust Versions of true and false

Create _hello/src/bin/true.rs_:

```rs
fn main() {
  std::process::exit(0);
}
```

- Use the `std::process::exit` function to exit the program with the value zero.

Test the previous function, add the following to _tests/cli.rs_:

```rs
#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}
```

> [!NOTE]
> The tests are not necessarily run in the same order they are declared in the
> code. This is because Rust is a safe language for writing concurrent
> code, which means code can be run across multiple threads. The testing takes
> advantage of this concurrency to run many tests in parallel, so the test
> results may appear in a different order each time you run them.

- Rust programs will exit with the value zero by default.

Repeat the process and create `src/bin/false.rs` + test.

```rs
#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure(); 1
}
```

We could write the `false.rs` like this:

```rs
fn main() {
    std::process::abort();
}
```

The previous tests only ensure that the programs exit correctly. To test program
output and ensure it actually prints the correct output we have to change the
`run` function:

```rust
use assert_cmd::Command;
use pretty_assertions::assert_eq; 1

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    let output = cmd.output().expect("fail"); 2
    assert!(output.status.success()); 3
    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8"); 4
    assert_eq!(stdout, "Hello, world!\n"); 5
}
```
