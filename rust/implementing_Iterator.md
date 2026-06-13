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

Creating an iterator of your own involves two steps:

1. Creating a `struct` to hold the iterator's state,

2. and then implementing `Iterator` for that `struct`.

Let's make an iterator named `Counter` which counts from `1` to `5`:

```rs
// An iterator that counts from one to five
struct Counter {
  count: usize,
}

// we want our count to start at one, so let's add a new() method to help
Note that we start
// `count` at zero, we'll see why in `next()`'s implementation below.
impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

// Then, we implement `Iterator` for our `Counter`

impl Iterator for Counter {
// we will be counting with usize
  type Item = usize;

// takes a mutable reference to self and returns an Optional item
  fn next(&mut self) -> Option<Self::Item> {
    // Increment our count. This is why we started at zero.
    self.count += 1;

    // Check to see if we've finished counting or not.
    if self.count < 6 {
      Some(self.count)
    } else {
      None
    }
  }
}

// And now we can use it!

let mut counter = Counter::new();

assert_eq!(counter.next(), Some(1));
assert_eq!(counter.next(), Some(2));
assert_eq!(counter.next(), Some(3));
assert_eq!(counter.next(), Some(4));
assert_eq!(counter.next(), Some(5));
assert_eq!(counter.next(), None);
```

## `for` loops and `IntoIterator`

```rs
let values = vec![1, 2, 3, 4, 5];

for x in values {
      println!("{x}");
      }
}
```

This will print the numbers one through five, each on their own line. But we
never called anything on our vector to produce an iterator. What gives?

There's a trait in the standard library for converting something into an
iterator: `IntoIterator`. This trait has one method, `into_iter`, which converts
the thing implementing `IntoIterator` into an iterator.

Let's check out the preceding for loop and see what the compiler converts it to:

```rs
let values = vec![1, 2, 3, 4, 5];

for x in values {
      println!("{x}");
      }
}
```

Rust de-sugars this into:

```rs
let values = vec![1, 2, 3, 4, 5];
{
  let result = match IntoIterator::into_iter(values) {
    mut iter => loop {
      let next;
      match iter.next() {
        Some(val) => next = val,
        None => break,
      };
      let x = next;
      let () = { println!("{x}"); };
    },
  };
  result
}
```

First, we call `into_iter()` on the value. Then, we match on the iterator that
returns, calling `next` over and over until we see a `None`. At that point, we
`break` out of the loop, and we're done iterating.

There’s one more subtle bit here: the standard library contains an interesting
implementation of `IntoIterator`:

```rs
impl<I: Iterator> IntoIterator for I
```

In other words, all `Iterator`s implement `IntoIterator`, by just returning
themselfes. This means two things:

1. If you're writing an `Iterator`, you can use it with a `for` loop.

2. If you're creating a collection, implementing `IntoIterator` for it will allow
your collection to be used with the `for` loop.


