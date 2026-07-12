# Smart Pointers and Interior Mutability

- rustacean-station (podcast)

- Module `std::cell`, shareable mutable containers.

Rust memory safety is based on this rule: Given an object `T`, it is only
possible to have one of the following:

- Several immutable references (`&T`) to the object (also known as aliasing)

- One mutable reference (`&mut T`) to the object (also known as mutability)

Sometimes it's required to have multiple references to an object and yet mutate
it.

- Values of `Cell<T>`, `RefCell<T>`, and `OnceCell<T>` types may be mutated
  through shared references (i.e. the `&T` type), whereas most Rust types can
  only be mutated through unique (`&mut T`) references. We say these cell types
  provide 'interior mutability' (mutable via `&T`), in contrast with typical
  Rust types that exhibit 'inherited mutability' (mutable only via `&mut T`).

## Cell<T>

`Cell<T>` implements interior mutability by moving values in and out of the
cell. That is, a `&T` to the inner value can never be obtained, and the value
itself cannot be directly obtained without replacing it with something else.

For types that implement `Copy`, the `get` method retrieves the current interior
value by duplicating it.

For types that implement `Default`, the `take` method replaces the current
interior value with `Default::default()` and returns the replaced value.

If you have a reference to a `Cell`, you can't give away that reference to a
different thread.

The benefit of `Cell` is that you can have multiple shared references to a
thing. You want to use `Cell` with cheap `Copy` types.

```rs
use std::cell::Cell;

let c = Cell::new(5);
```

`Cell` does not implement `Sync` (it's single threaded).

You want to use `Cell` with small `Copy` types. You can only get the value out
of a `Cell` if either you have a mutable reference to it, in which case you
probably don't need a cell, or if the value is `Copy`.

When we implement `!Sync` for `Cell<T>`, we're saying that you can never share a
`Cell` across threads.

The `Cell` type allows you to modify a value through a shared reference because
no other threads have a reference to it and so you can't have multiple
concurrent operations and because you haven't given out a reference into the
value you store and therefore you can replace it just fine.

`get` only ever returns a copy, never a reference.

```rs
use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied from `UnsafeCell`
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know no-one else is concurrently mutating self.value because `!Sync`
        // SAFETY: we know we're not invalidating any references, because we niver give any out
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no-one else is modifying this value, since only this thread
        // can butate because !Sync
        unsafe { *self.value.get() }
    }
}

mod test {
    use std::thread;

    use super::Cell;

    #[test]
    fn bad() {
        use std::sync::Arc;
        let x = Arc::new(Cell::new(42));
        let x1 = Arc::clone(&x);

        std::thread::spawn(move || {
            x1.set(43);
        });
        let x2 = Arc::clone(&x);
        std::thread::spawn(move || {
            x2.set(44);
        });
    }

    #[test]
    fn bad2() {
        let x = Cell::new("hello");
        let first = x.get();
        x.set("goodbye");
        dbg!("{}", first);
    }
}
```

You are not allowed to cast a shared reference to an exclusive reference except
through `UnsafeCell`.

## RefCell

RefCell lets you check at runtime whether anyone else is mutating the value.

A mutable memory location with dynamically checked borrow rules. Normally borrow
checking is checked at compile time. This let's you check at runtime.

`RefCell<T>` uses Rust's lifetimes to implement "dynamic borrowing", a process
whereby one can claim temporary, exclusive, mutable access to the inner value.
Borrows for `RefCell<T>`s are tracked at _runtime_, unlike Rust's native
reference types which are entirely tracked statically, at compile time.

An immutable reference to a `RefCell`s inner value (`&T`) can be obtained with
`borrow`, and a mutable borrow (`&mut T`) can be obtained with `borrow_mut`.

A `RefCell<T>` type represents single ownership over the data it holds. With
references and `Box<T>`, the borrowing rules' invariants are enforced at compile
time. With `RefCell<T>`, these invariants are enforced at runtime. If you break
the rules, your program will panic and exit.

Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate
the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.

## Using Interior Mutability

With the standard borrowing rules, when you have an immutable value, you can't
borrow it mutably. This won't compile:

```rs
fn main() {
  // assign 5 to x immutably
  let x = 5;
  // attempt to mutably borrow an immutable variable
  let y = &mut x;
}
```

## Tracking Borrows at Runtime

The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers
are currently active. Every time we call borrow, the `RefCell<T>` increases its
count of how many immutable borrows are active. When a `Ref<T>` value goes out
of scope, the count of immutable borrows goes down by 1. Just like the
compile-time borrowing rules, `RefCell<T>` lets us have many immutable borrows
or one mutable borrow at any point in time.

## Allowing Multiple Owners of Mutable Data

It's common to use `RefCell<T>` with `Rc<T>`. `Rc<T>` lets you have multiple
owners of some data, but only gives immutable access to that data. If you have
an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can have multiple
owners _and_ that you can mutate!

```rs
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
```
