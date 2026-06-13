# The Option<T> match Pattern

We can also handle `Option<T>` using `match`, instead of
comparing coins, we'll compare the variants of `Option<T>`.

Let's say we want to write a function that takes an `Option<i32>` and, if there's a value inside, adds 1 to that value.
If there isn't a value inside, the function should return
the `None` value and not attempt to perform any operations.

```rs
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```
