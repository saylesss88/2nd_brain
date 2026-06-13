# Iterators

`Iterator` is a trait.

```rs
pub trait Iterator {
  type Item;  // associated type (only 1 implementation to choose)
  fn next(&mut self) -> Option<Self::Item>;
}
```

Why the above and not this:

```rs
trait Iterator<Item> {
  fn next(&mut self) -> Option<Item>;
}
```

Both work, you use an associated type if you know there is only one iterator
type for it. Iterators typically only have one item type.

When you write something like:

```rs
fn main() {
  for x in vec!["a","b","c"] {

  }
}
```

What happens under the hood is interesting, there aren't really `for` loops when
you get below a certain level of syntactic sugar.

It first turns into an `iter`.

```rs
fn main() {
  let mut iter = vec!["a", "b", "c"].into_iter();
  while let Some(e) = iter.next() {
  }
}
```

This is a desugaring of what we wrote before. The for loop can go away and be
replaced with a `while let` loop that keeps calling `next()` while there's still
`Some`.

- `into_iter` is a separate trait

There is a big difference between these:

```rs
fn main() {
  let vs = vec![1, 2, 3];
  for v in vs {
    // consumes vs, and gives you owned access to v
    // it's not going to auto-borrow unless you ask it to
  }

  for v in vs.iter() {
    // borrows vs, & to v
  }

  for v in &vs {
    // equivalent to vs.iter()
  }
}
```

- `flatten` takes an iterator,

```rs
fn flatten(iter: I) -> Flatten<I> {
  Flatten::new(iter)
}

pub struct Flatten<O> {
    outer: O,
}

impl<O> Flatten<O> {
  fn new(iter: O) -> Self {
      Flatten { outer: iter }
  }
}

impl<O> Iterator for Flatten<O> 
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
      None
    }
}
```
