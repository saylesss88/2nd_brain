# References

A _reference_ is a value that stands in place for another value. For example,
say variable `a` is a large array that's costly to duplicate. In some sense, a
reference `r` is a cheap copy of `a`. But instead of creating a duplicate, the
program stores `a`'s address in memory. When the data from `a` is required, `r`
can be dereferenced to make `a` available.

Creating a reference to a large array:

```rs
fn main() {
  let a = 42;
  let r = &a;
  let b = a + *r;

  println!("a + a = {b}");
}
```

References are created with the _reference operator_ (`&`) and dereferencing
occurs with the _dereference operator_ (`*`). These operators act as _unary
operators_, meaning that these only take one operand.

Searching for an integer in an array of integers:

```rs
fn main() {
  let needle = 0o204;
  let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

  for item in &haystack {
    if *item == needle {
      println!("{item}");
    }
  }
}
```

Each iteration changes the value of `item` to refer to the next item within
`haystack`.
