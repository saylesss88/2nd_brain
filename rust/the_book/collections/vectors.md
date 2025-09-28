---
id: vectors
aliases: []
tags: []
---

# Storing Lists of Values with Vectors

The vector collection type `Vec<T>`, known as _vector_.

- Vectors allow you to store multiple values in a single data structure that
  puts all the values next to each other in memory.

    - Vectors can only store values of the same type. Useful when you have a list
      of items, such as the lines of text in a file or the prices of items in a
      shopping cart.

## Creating a New Vector

To create a new empty vector, we call the `Vec::new` function:

```rust
let v: Vec<i32> = Vec::new();
```

- Creating a new, empty vector to hold values of type `i32`

- Since this is an empty vector we annotate the type, rust cant infer the type
  of a non existant value. More often, you'll create a `Vec<T>` with initial
  values and Rust will infer the type of value you want to store.

- Rust has the `vec!` macro, which will create a new vector that holds the
  values you give it.

```rust
let v = vec![1, 2, 3]
```

- Because we've given initial `i32` values, Rust can infer that the type of `v`
  is `Vec<i32>`, and the type annotation is unnecessary.

## Updating a Vector

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

- The numbers inside are all the default `i32`

### Reading Elements of Vectors

There are two ways to reference a value stored in a vector: via indexing or by
using the `get` method:

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
  Some(third) => println!("The third element is {third}"),
  None => println!("There is no third element."),
}
```

- Using `&` and `[]` gives us a reference to the element at the index value.

- When we use the `get` method with the index passed as an argument, we get an
  `Option<&T>` that we can use with `match`.

```rust
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

- The first `[]` method will cause the program to panic, this method is best
  when you want your program to crash if there's an attempt to access an element
  past the end of the vector.

- When the `get` method is passed an index that is outside the vector, it
  returns `None` without panicking. Use this if accessing an element beyond the
  range of the vector may happen occasionally under normal circumstances.

    - Your code will then have logic to handle having either `Some(&element)` or
      `None`. You could tell the user how many items are in the current vector and
      give them another chance to enter a valid value. (more user friendly).

## Iterating Over the Values in a Vector

To access each element in a vector in turn, we would iterate through all of the
elements rather than use indices to access one at a time.

A `for` loop to get immutable references to each element in a vector of `i32`
values and print them out:

```rust
let v = vec![100, 32, 57];
for i in &mut v {
  println!("{i}");
}
```

We can also iterate over mutable references to each element in a mutable vector
in order to make changes to all the elements. The for loop will add `50` to each
element:

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
  *i += 50;
}
```

- To change the value that the mutable reference refers to, we have to
  dereference it with `*` to get the value in `i` before we can use the `+=`
  operator.

    - Iterating over a vector, whether immutably or mutably, is safe because of
      the borrow checker's rules. The reference to the vector that the `for` loop
      holds prevents simultaneous modification of the whole vector.

[[using_an_enum_to_store_multiple_types.md]]
