# for loops

The `for` loop is the workhorse of iteration in Rust.

```rs
for item in container {
  // ...
}
```

This makes each successive element in `container` available as `item`. Once the
block ends, accessing the container another time becomes invalid. Even though
the `container` variable remains within local scope, its _lifetime_ has ended.

When you need to reuse `container` later in your program, use a reference:

```rs
for item in &container {
  // ...
}
```

If you need to modify each `item` during the loop, you can use a _mutable
reference_ with `mut`:

```rs
for item in &mut container {
  // ...
}
```

Rusts `for` loop construct is expanded to different method calls by the
compiler:

| Shorthand                   | Equivalent to                                   | Access     |
| --------------------------- | ----------------------------------------------- | ---------- |
| for item in collection      | for item in IntoIterator::into_iter(collection) | Ownership  |
| for item in &collection     | for item in collection.iter()                   | Read-only  |
| for item in &mut collection | for item in collection.iter_mut()               | Read-write |

## Anonymous Loops

When a local variable is not used within a block, by convention, you'll use an
underscore (`_`). Using this pattern in conjunction with the _exclusive range
syntax_ (n..m) and _inclusive range syntax_ (n..=m) makes it clear that the
intent is to perform a loop for a fixed number of times.

```rs
for _ in 0..10 {
  // ...
}
```

## Avoid Managing an Index Variable

```rs
let collection = [1, 2, 3, 4, 5];
for i in 0..collection.len() {
 let item = collection[i];
 // ...
}
```

This is legal Rust. It's also essential in cases when iterating directly over
`collection` via `for item in collection` is impossible. However, it's generally
discouraged. The manual approach introduces two problems with this:

- _Performance_: Indexing values with the `collection[index]` syntax incurs
  runtime costs for bounds checking. (Rust checks that `index` currently exists
  within `collection` as valid data). Those checks are unnecessary when
  iterating directly over `collection`.

## Continue: Skipping the Rest of the Current Iteration

```rs
for n in 0..10 {
  if n % 2 == 0 {
    continue;
  }
}
```

## Break: Aborting a Loop

```rs
for (x, y) in (0..).zip(0..) {
  if x + y > 100 {
    break;
  }
}
```

## While: Looping Until a Condition Changes Its State

```rs
let mut samples = vec![];
while samples.len() < 10 {
 let sample = take_sample();
 if is_outlier(sample) {
 continue;
 }
 samples.push(sample);
}
```
