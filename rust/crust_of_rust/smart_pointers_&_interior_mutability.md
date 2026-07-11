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
