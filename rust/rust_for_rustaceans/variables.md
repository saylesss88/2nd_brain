# Variables

- You can have only one mutable borrow at a time.

- You can have as many immutable borrows as you want at a time.

- You can't have an exclusive and a shared use of a value at the same time.

```rs
fn main() {
    let mut x;
    x = 42;
    let y = &x;
    x = 43;
    assert_eq!(*y, 42)
}
```

Output:

```text
error[E0506]: cannot assign to `x` because it is borrowed
 --> src/main.rs:5:5
  |
4 |     let y = &x;
  |             -- `x` is borrowed here
5 |     x = 43;
  |     ^^^^^^ `x` is assigned to here but it was already borrowed
6 |     assert_eq!(*y, 42)
  |     ------------------ borrow later used here

warning: value assigned to `x` is never read
 --> src/main.rs:5:5
  |
5 |     x = 43;
  |     ^^^^^^
```

If we remove the last `assert_eq` statement, the code compiles fine because the
shared flow would end at `let y = &x`. When the exclusive flow is checked at
`x = 43;`, no conflicting flows would exist!

## Low-Level Model

Variables name memory locations that may or may not hold legal values. You can
think of a variable as a "value slot". When you assign to it, the slot is filled,
and its old value (if it had one) is dropped and replaced. When you access it
the compiler checks that the slot isn't empty, as that would mean the variable
is uninitialized or its value has been moved.

A pointer to a variable refers to the variable's backing memory and can be
dereferenced to get at its value.

For example, in the statement `let x: usize`, the variable `x` is a name for a
region of memory on the stack that has room for a value the size of a `usize`,
though it doesn't have a well-defined value (its slot is empty). If you assign
a value, such as with `x = 6`, that region of memory will then hold the bits
representing the value `6`. `&x` does not change when you assign to `x`.

If you declare multiple variables with the same name, they still end up with
different memory backing them.
