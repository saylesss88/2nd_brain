# Functions, Closures, and their Traits

- Rust For Rustaceans

```rs
fn main() {

}
```

```rs
fn main() {
    println!("Hello, world!");
    let x = bar;
}

fn bar() {}
```

- `x` here has a type, what is it? It's a function item. A 0 sized value that's
  only carried around at compile time.

```rs
fn main() {
    println!("Hello, world!");
    let x = bar::i32;
    x = bar::<u32>;
}

fn bar<T>() {}
```

- The signature of `bar` is that it takes no arguments and returns nothing.

```rs
fn main() {
    println!("Hello, world!");
    let x = bar::<i32>;
    println!("{}", std::mem::size_of_val(&x));
    // x = bar::<u32>;
}

fn bar<T>() {}
```

Hello, world! 0

```rs
fn main() {
    println!("Hello, world!");
    let x = bar::<i32>;
    println!("{}", std::mem::size_of_val(&x));
    // x = bar::<u32>;
    baz(bar::<u32>); // These are both different instances of bar with diff types
    baz(bar::<i32>); // All work,
    baz(bar::<f64>);
}

fn bar<T>(_: u32) -> u32 {
    0
}

fn baz(f: fn(u32) -> u32) {
    println!("{}", std::mem::size_of_val(&f));
}
```

**Output**:

```text
Hello, world!
0
8
8
```

- The size of the value is 8 here because it's an actual function pointer.

- `baz` here takes a function pointer as an argument.

- In both cases the compiler coerces the function item type into a function
  pointer type, so this function can be called.

- function items are coercible into a function pointer

- A function item uniquely identifies a particular instance of a function.

- A function pointer is a pointer to a function with a given signature and you
  can turn one into the other, but not go the other way around.

```rs
fn main() {
    println!("Hello, world!");
    let x = bar::<i32>;
    println!("{}", std::mem::size_of_val(&x));
    // x = bar::<u32>;
    baz(bar::<u32>);
    baz(bar::<i32>);
    baz(bar::<f64>);
}

fn bar<T>() {}

fn baz(f: fn()) {
    println!("{}", std::mem::size_of_val(&f));
}
```

This still works.

