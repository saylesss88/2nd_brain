# StrSplit and lifetimes

`StrSplit` lets you take a string and split it by the string and walk the splits
of that string.

Lifetimes are like types.

Add the following to the top of every project:

```rs
//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
```

Use warn not deny because these frequently change.

```rs
pub struct StrSplit {}

impl StrSplit {
  pub fn new(haystack: &str, delimiter: &str) -> Self {}
}
```

`new` is basically saying that we want to split `haystack` by `delimiter`.

- [Trait Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

```rs
pub trait Iterator {
  type Item;

  // Required method
  fn next(&mut self) -> Option<Self::Item>;
}
```

A trait for dealing with iterators.

- [Implementing Iterator](https://doc.rust-lang.org/std/iter/index.html#implementing-iterator)

Most the magic happens in the `next()` method. We're going to find where the
next delimiter appears in the remainder and then we're going to chop off that
part of the string, that's what we're going to return. And we're going to set
the remainder to what remains after the delimiter.

```rs
impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        // if let Some because there could always not be a delimiter that matches
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            // from the start to the next delimiter
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            // TODO: Bug
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
        }
    }
}
```

When you use a `for` loop over `StrSplit`, Rust calls the `next()` method
repeatedly under the hood. The method looks at `self.remainder` and matches one
of three scenarios:

Scenario A: The delimiter is found (`if let Some(...)`)

Imagine `remainder` is `"a b c d e"` and the `delimiter` is `" "`.

1. `self.remainder.find(" ")` finds the space at index `1`.

2. `let until_delimiter = &self.remainder[..1];` grabs everything before the
   space: `"a"`.

3. `self.remainder = &self.remainder[(1 + 1)..]` updates the remainder to start
   _after_ the space, skipping the length of the delimiter. The new remainder is
   now `"b c d e"`.

4. It yields `Some("a")`.

Scenario B: No delimiter found, but the text remains (`else`)

Eventually, your `remainder` will just be `"e"`

1. `find(" ")` returns `None` (there are no more spaces)

2. The code steps into the `else` block

3. `let rest = self.remainder;` captures `"e"`

4. `self.remainder = "";` clears out the remainder so it's completely empty for
   the next round.

5. It yields `Some("e")`.

Scenario C: The string is completely drained (else if)

The next time `next()` is called, remainder is `""`.

1. `self.remainder.is_empty()` returns `true`.

2. It returns `None`, which tells the for loop: "We are all done, stop looping!"

## <'_> Anonymous Lifetime

Use the anonymous lifetime when you can.

If you have a signature like this:

```rs
fn foo<'x, 'y>(x: &'x str, y: &'y str) -> &'x str {}
```

It can be simplified to:


```rs
fn foo(x: &str, y: &'_ str) -> &'_ str {}
```


## Complete Program

```rs
// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug, PartialEq, Eq)]
pub struct StrSplit<'a> {
    remainder: &'a str, // remaining string
    delimiter: &'a str, // what are we splitting by
}

// You can only use the StrSplit for as long as the input values to new are valid `haystack` and `delimiter`.
impl<'a> StrSplit<'a> {
    // I want to split `haystack` by `delimiter`
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

// let x: StrSplit;
// for part in x {}
//
impl<'a> Iterator for StrSplit<'a> {
    // this pointer points to `self.remainder`
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        // If the result of calling next() is Some, store it in next_delim
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            // from the start to the next delimiter
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            // TODO: Bug
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            //  &'a str      &'static str
            // we can assign &'static str because it's got a longer lifetime
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");

    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));

    // for letter in StrSplit::new(haystack, " ") {
    // a
    // b
    // c
    // d
    // e
    // }
}
```
