# Closures

A closure is called a closure because it closes over its environment — it can
capture variables from the surrounding scope.

Closures can be passed as arguments to functions, returned as values from
functions, and assigned to variables.

## The three Fn traits

- `Fn` — borrows `self` shared (`&self`). Can be called multiple times, even
  concurrently.
- `FnMut` — borrows self exclusively (`&mut self`). Can be called multiple
  times, but not concurrently.
- `FnOnce` — takes ownership of `self`. Can only be called once.

They form a hierarchy: every `Fn` implements `FnMut`, and every `FnMut`
implements `FnOnce`. So a function accepting `FnOnce` is the most flexible — it
accepts all three.

## Function pointers vs closures

A function pointer (`fn()`) has no state — it's a standalone chunk of code that
doesn't reference anything outside itself. No lifetimes, no self. Because of
this, function pointers implement all three traits (`Fn`, `FnMut`, `FnOnce`).

```rs
fn baz(f: fn()) {
    println!("{}", std::mem::size_of_val(&f)); // always pointer-sized
}
```

**Non-capturing closures**

```rs
let f = |x: i32, y: i32| x + y;
```

This only uses its own arguments - it captures nothing from the environment.
Non-capturing closures are coercible to function pointers (`fn()`), and
implement all three `Fn` triats.

**Capturing closures**

When a closure references something from its enclosing scope, the compiler
generates a hidden struct to hold those captured values. You can think of it
like this:

**Shared borrow -> implements `Fn`:**

```rs
let z = String::new();
let f = || println!("{}", z);

// Compiler generates roughly:
struct FClosure<'a> { z: &'a String }
impl<'a> Fn() for FClosure<'a> {
    fn call(&self) { println!("{}", self.z); }
}
```

\*\*Mutable borrow -> implements `FnMut` (not `Fn`):

```rs
let mut z = String::new();
let f = || z.clear();

// Compiler generates roughly:
struct FClosure<'a> { z: &'a mut String }
impl<'a> FnMut() for FClosure<'a> {
  fn call(&mut self) { self.z.clear(); }
}
```

Can't implement `Fn` because `Fn:call` only gets `&self`, and you can't get
`&mut String` out of `&self`.

\*\*Move into closure -> implements `FnOnce`:

```rs
let z = String::new();
let f = || drop(z); // z is moved into the closure

// Compiler generates roughly:
struct FClosure { z: String } // owned, no lifetime needed
impl FnOnce() for FClosure {
    fn call(self) { drop(self.z); }
}
```

Can only be called once - `z` gets moved out of the struct on the first call, so
there's nothing left for a second call.

## `move` closures

Normally the compiler decides _how_ to capture based on what you do with the
value in the closure body. `move` forces everything to be moved in regardless:

```rs
let z = String::new();
let f = move || println!("{}", z); // z is owned by the closure
```

This is necessary when the closure needs to outlive the scope where the captures
live - like returning a closure from a function:

```rs
fn make_fn() -> impl Fn() {
    let z = String::new();
    move || {
        println!("{}", z);
    }
}
```

If multiple variables are in scope, `move` moves all of them:

```rs
fn make_fn() -> impl Fn() {
    let x = String::new();
    let z = String::new();
    move || {
        println!("{}", x); // both moved in
        println!("{}", z);
    }
}
```

## Dynamic dispatch

A trait object points to both an instance of a type implementing our specified
trait and a table used to look up trait methods on that type at runtime.

We create a trait object by specifying some sort of pointer, such as a reference
or a `Box<T>` smart pointer, then the `dyn` keyword, and then specifying the
relevant trait. We can use trait objects in place of a generic or concrete type.
Their specific purpose is to allow abstraction across common behavior.

```rs
pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>, // stand-in for any type inside a `Box`
                                      // that implements the `Draw` trait.
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    // code to draw a button
  }
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    // code to draw a select box
  }
}
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

Definition of the `Screen` struct with a `components` field holding a vector of
trait objects that implement the `Draw` trait.

When you need to store or pass closures of different types, use a trait object:

```rs
fn hello(f: Box<dyn Fn()>) {
  f()
}
fn hello(f: &dyn Fn()) {}
```

`Box<dyn Fn()>` heap-allocates the closure and erases its concrete type. Use
this when you need to store closures in a struct or return a different closure
type from a branch.

It used to be `Box<dyn Fn>` did not implement any of the `Fn` traits.

```rs
impl FnOnce() for Box<dyn FnOnce()> {
  fn call(&self) {
      let x: dyn FnOnce() = self.0;
      x.call()
  }
}
```

`dyn` in general is not sized.

### Resources

- [crust of rust Functions, Closures, and their Traits](https://www.youtube.com/watch?v=dHkzSZnYXmk)

- [eventhelix Closures](https://www.eventhelix.com/rust/rust-to-assembly-return-impl-fn-vs-dyn-fn/)
