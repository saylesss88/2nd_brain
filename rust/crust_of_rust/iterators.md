# Iterators

`Iterator` is a trait that has two primary things to be concerned with. It has
an associated type called `Item` and an associated method called `next()`.

- `Item` is the type that's going to be returned by the iterator.

- Inherant methods are implemented on the type rather than a trait.

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
    // borrows vs, gives reference to v
  }

  for v in &vs {
    // equivalent to vs.iter()
  }
}
```

- `flatten` takes an iterator, and calls `into_iter` on each of it's items
  creating another iterator of iterators. When the inner iterator is exhausted,
  we'll then move on to the next outer item etc.

```rs
let nested = vec![vec![1, 2, 3], vec![4, 5], vec![6]];
let flat: Vec<i32> = nested.into_iter().flatten().collect();
// [1, 2, 3, 4, 5, 6]
```

Calling `nested.into_iter()` above, turns a `Vec<Vec<i32>` into an iterator of
`Vec<i32>`. `flatten` is a method on iterators, so you need an iterator to call
it.

1. `nested.into_iter()` -> an iterator yielding `Vec<i32>` items: `vec![1,2,3]`,
   `vec![4,5]`, `vec![6]`

2. `.flatten()` wraps that, and each time you call `.next()` on it, internally
   it:

- Takes the current item (a `Vec<i32>`)
- Calls `into_iter()` on that item to get an inner iterator
- Pulls from the inner iterator until exhausted
- Then goes back to the outer iterator for the next item, and repeats

This is why you don't have to write `.map(|v| v.into_iter())` yourself.

- This is why the requirement is "the item type must implement `IntoIterator`"
- `flatten()` is calling `into_iter()` on each item it receives from the outer
  iterator, so each item needs to support that call.

```rs
fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.inner = None;
            }
            let next_inner_iter = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_iter)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn empty_wide() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0)
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
    }
    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
    }
}
```

- Try to implement `FlatMap` to see if you understood this.

## extension traits

```rs
pub trait IteratorExt: Iterator {
  fn flatten(self) -> Flatten<Self> where Self::Item: IntoIterator {
    flatten(self)
  }
}

impl<T> IteratorExt for T where T: Iterator {
  
  fn flatten(self) -> Flatten<Self>
   where
       Self::Item: IntoIterator
  {
    flatten(self)
  }
}
```
