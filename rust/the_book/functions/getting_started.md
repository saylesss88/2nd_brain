# Functions

We can define _parameters_, which are special variables that are part of a
function's signature.

When a function has parameters, you can provide it with concrete values for
those parameters.

```rs
fn main() {
  another_function(5);
}

fn another_function(x: i32) {
  println!("The value of x is: {x}");
}
```

```rs
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: { }", x);
}
```
