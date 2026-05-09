# Writing Automated Tests

_Correctness_ in our program is the extent to which our code does what we intend
it to do.

Rust’s type system shoulders a huge part of this burden, but the type system
cannot catch everything. As such, Rust includes support for writing automated
software tests.

We can write tests that assert, for example, that when we pass `3` to the
`add_two` function, the returned value is `5`. We can run these tests whenever
we make changes to our code to make sure any existing correct behavior has not
changed.

## How to Write Tests

Tests are Rust functions that verify that the non-test code is functioning in
the expected manner. The bodies of test functions typically perform these three
actions:

- Set up any needed data or state.
- Run the code you want to test.
- Assert that the results are what you expect.

## Structuring Test Functions

A simple Rust test is just a function that's annotated with the `test`
attribute.

When you run your tests with the `cargo test` command, Rust builds a test runner
binary that runs the annotated functions and reports on whether each test
function passes or fails.

When you run `cargo new <lib-name> --lib`, cargo automatically generates an
example test template in `src/lib.rs`.

```rs
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

The `cargo test` command runs all tests in our project.

We can also pass an argument to the `cargo test` command to run only tests whose
name matches a string; this is called _filtering_. Example:
`cargo test it_works`

The `0 measured` statistic is for benchmark tests that measure performance.
Benchmark tests are, as of this writing, only available in nightly Rust. See
[the documentation about benchmark tests](https://doc.rust-lang.org/unstable-book/library-features/test.html)
to learn more.
