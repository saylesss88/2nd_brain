# Deref Trait

The following will fail with "type MyBox<{integer}>" cannot be dereferenced.

```rs
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

Our `MyBox<T>` type can’t be dereferenced because we haven’t implemented that
ability on our type. To enable dereferencing with the `*` operator, we implement
the `Deref` trait.

## Implementing the Deref Trait

```rs
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

Adding this to the above example allows us to dereference `y` and compiles.

We fill the body of the `deref` method with `&self.0` which is the first value
in a tuple struct.

Without the `Deref` trait, the compiler can only dereference `&` references. The
`deref` method gives the compiler the ability to take a value of any type that
implements `Deref` and call the `deref` method to get a reference that it knows
how to dereference.

When we entered `*y`, behind the scenes Rust actually ran this:

```rs
*(y.deref())
```

Rust substitutes the \* operator with a call to the `deref` method and then a
plain dereference so that we don’t have to think about whether or not we need to
call the `deref` method. This Rust feature lets us write code that functions
identically whether we have a regular reference or a type that implements
`Deref`.

### Deref Coercion

_Deref Coercion_ converts a reference to a type that implements the `Deref`
trait into a reference to another type. For example, `deref` coercion can
convert `&String` to `&str` because `String` implements the `Deref` trait such
that it returns `&str`.

```rs
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {}
```

We can call the hello function with a string slice as an argument, such as
`hello("Rust");`, for example. Deref coercion makes it possible to call `hello`
with a reference to a value of type `MyBox<String>`.

```rs
fn main() {
  let m = MyBox::new(String::from("Rust"));
  hello(&m);
}
```

Here we’re calling the `hello` function with the argument `&m`, which is a
reference to a `MyBox<String>` value. Because we implemented the Deref trait on
`MyBox<T>` , Rust can turn `&MyBox<String>` into `&String` by calling `deref`.

If rust didn't implement deref coercion, we would have to write:

```rs
fn main() {
  let m = MyBox::new(String::from("Rust"));
  hello(&(*m)[..]);
}
```

The `(*m)` dereferences the `MyBox<String>` into a `String`. Then, the `&` and
`[..]` take a string slice of the `String` that is equal to the whole string to
match the signature of hello.

```rs
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
```
