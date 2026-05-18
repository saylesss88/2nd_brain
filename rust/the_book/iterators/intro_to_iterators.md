# Processing a Series of Items with Iterators

The iterator pattern allows you to perform some task on a sequence of items in
turn. An iterator is responsible for the logic of iterating over each item and
determining when the sequence has finished. When you use iterators, you don’t
have to reimplement that logic yourself.

- Iterators are _lazy_, meaning they have no effect until you call methods that
  consume the iterator to use it up.

The following creates an iterator over the items in `v1` by calling the `iter`
method defined on `Vec<T>`:

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
```

Once you create an iterator, you can use it in a variety of ways.
