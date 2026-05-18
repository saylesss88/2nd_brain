# Running Code on Cleanup with the Drop Trait

`Drop` lets you customize what happens when a value is about to go out of scope.

You specify the code to run when a value goes out of scope by implementing the
`Drop` trait. The `Drop` trait requires you to implement one method named `drop`
that takes a mutable reference to `self`.

`CustomSmartPointer` structs functionality is to print
`Dropping CustomerSmartPointer!` when the instance goes out of scope:

```rs
struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

fn main() {
  let c = CustomSmartPointer {
    data: String::from("my stuff"),
  };
  let d = CustomSmartPointer {
    data: String::from("other stuff"),
  };
  println!("CustomSmartPointers created");
}
```

The `Drop` trait is included in the prelude, so we don’t need to bring it into
scope. We implement the Drop trait on `CustomSmartPointer` and provide an
implementation for the `drop` method that calls `println!`. The body of the drop
method is where you would place any logic that you wanted to run when an
instance of your type goes out of scope. We’re printing some text here to
demonstrate visually when Rust will call `drop`.

We created two instances of `CustomSmartPointer` and then print
`CustomSmartPointers created`. At the end of `main`, our instances go out of
scope, and Rust will call the code we put in the `drop` method, printing our
final message. Note that we didn't need to call the `drop` method explicitly.

```bash
cargo run
CustomSmartPointers created
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

Rust automatically called `drop` for us when our instances went out of scope,
calling the code we specified. Variables are dropped in the reverse order of
their creation, so `d` was dropped before `c`. You typically specify the cleanup
code that your type needs rather than a print message.

## Disabling `drop`

You might want to force `drop` so that it releases the lock so that other code
in the same scope can acquire the lock. Rust doesn't let you call the `Drop`
trait's `drop` method manually; instead, call the `std::mem::drop` function
provided by the standard library.

```rs
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
  let c = CustomSmartPointer {
    data: String::from("some data"),
  };
  println!("CustomSmartPointer created");
  // c.drop(); fails, can't call method
  drop(c); // std::mem::drop is in the prelude so this works
  println!("CustomSmartPointer dropped before the end of main");
}
```

```bash
$ cargo run
   Compiling drop-example v0.1.0 (file:///projects/drop-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/drop-example`
CustomSmartPointer created
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main
```
