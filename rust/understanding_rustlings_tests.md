# Rustlings Tests

```rs
// TODO: Fix the compiler error on this function.
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else if food == "potato" {
        "I guess I can eat that."
    } else {
        "No thanks!"
    }
}

fn main() {
    // You can optionally experiment here.
}

// TODO: Read the tests to understand the desired behavior.
// Make all tests pass without changing them.
// conditional compilation attribute; if the condition is true, the item stays;
// if false, the item is removed from the compiled program
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // This means that calling `picky_eater` with the argument "strawberry" should return "Yummy!".
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}
```

## Attributes

`#[cfg(test)]` and `#[test]` solve two different problems: `#[cfg(test)]`
controls whether some code is compiled at all, while `#[test]` marks a
particular function as a test for the test runner to execute. ​

`cfg` meaning:

`cfg` is short for “configuration,” and `#[cfg(...)]` is Rust’s conditional
compilation attribute: if the condition is true, the item stays; if it’s false,
the item is removed from the compiled program. In `#[cfg(test)]`, the condition
is the built-in test configuration, which is enabled when you run cargo test, so
your `mod tests { ... }` only exists in test builds. ​

Why both `#[cfg(test)]` and `#[test]`:

`#[cfg(test)]` prevents test-only code (and its dependencies/imports) from being
part of normal builds like `cargo build`/`cargo run`. `#[test]` is what the Rust
test harness uses to discover “this function is a test case; run it”; without
`#[test]`, it’s just a regular helper function inside the tests module. ​

- The `#[ ... ]` syntax is an _attribute_ applied to the next item, and it can
  carry parameters like `#[cfg(test)]` or `#[allow(unused)]`. The `#![ ... ]`
  form is an "inner attribute" that applies to the enclosing scope.

---

## mod tests

`mod tests` is an in-file test module, and `use super::*;` brings names from the
parent module (like `picky_eater`) into the `tests` module's scope so you can
call them directly.

Without `super::*;` you'd need to qualify the path to items in the parent module
(e.g., `super::picky_eater("strawberry")`)

---

## assert_eq!

`assert_eq!(left, right)` checks that the two values are equal; if not, the test
fails and the harness prints a diff-style message showing "left" (what the code
returned) vs "right" what the test expected.
