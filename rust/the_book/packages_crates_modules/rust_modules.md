# Rust Modules

In Rust, all items (functions, methods, structs, enums, modules, and constants)
are private to parent modules by default. If you want to make an item like a
function or struct private, put it in a module.

Two ways to call `add_to_waitlist` from a new function, `eat_at_restaurant`,
defined in the crate root.

The `eat_at_restaurant` function is part of our library crate's public API, so
we mark it with the `pub` keyword:

```rs
// This won't compile!!
mod front_of_house {
  mod hosting {
    fn add_to_waitlist() {}
  }
}

pub fn eat_at_restaurant() {
  // Absolute path
  crate::front_of_house::hosting::add_to_waitlist();

  // Relative path
  front_of_house::hosting::add_to_waitlist();
}
```

The `add_to_waitlist` function is defined in the same crate as
`eat_at_restaurant`, which means we can use the `crate` keyword to start an
absolute path.

If you plan to share your library crate so that other projects can use your
code, your public API is your contract with users of your crate that determines
how they can interact with your code.

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

```text
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
  |                            |
  |                            private module
  |
note: the module `hosting` is defined here
 --> src/lib.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
   |                     |
   |                     private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` (lib) due to 2 previous errors
```

The error messages say that module `hosting` is private.

Items in a parent module can't use the private items inside child modules, but
items in child modules can use the items in their ancestor modules.

## Exposing Paths with the pub Keyword

```rs
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

// -- snip --
```

We had to add `pub` to the function as well because making the module public
doesn't make its contents public. The `pub` keyword on a module only lets code
in its ancestor modules refer to it, not access its inner code. Because modules
are containers, making them public without also making their contents public
doesn't do much.

> **Best Practices for Packages with a Binary and a Library**
>
> Typically, packages with this pattern of containing both a library and a
> binary crate will have just enough code in the binary crate to start an
> executable that calls code defined in the library crate. This lets other
> projects benefit from the most functionality that the package provides because
> the library crate’s code can be shared.
>
> The module tree should be defined in src/lib.rs. Then, any public items can be
> used in the binary crate by starting paths with the name of the package. The
> binary crate becomes a user of the library crate just like a completely
> external crate would use the library crate: It can only use the public API.
> This helps you design a good API; not only are you the author, but you’re also
> a client!

## Starting Relative Paths with super

```rs
fn deliver_order() {}

mod back_of_house {
  fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
  }

  fn cook_order() {}
}
```

The `fix_incorrect_order` function is in the `back_of_house` module, so we can
use `super` to go to the parent module of `back_of_house`, which in this case is
`crate`, the root. From there, we look for `deliver_order` and find it. We think
the `back_of_house` module and the `deliver_order` function are likely to stay
in the same relationship to each other and get moved together should we
refactor. Therefore, we used `super` so we'll have fewer places to update code
in the future if this code gets moved to a different module.
