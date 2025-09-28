---
id: using_an_enum_to_store_multiple_types
aliases:
    - using_an_enum_to_store_multiple_types
tags: []
---

# using_an_enum_to_store_multiple_types

Vectors can only store values that are the same type which can be inconvenient.

Fortunately, the variants of an enum are defined under the same enum type, so
when we need one type to represent elements of different types, we can define
and use an enum!

```rust
fn main() {
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];
}
```

- Rust needs to know what types will be in the vector at compile time so it
  knows exactly how much memory on the heap will be needed to store each element.

    - We also must be explicit about what types are allowed in this vector.

    - Using an enum plus a `match` expression means that Rust will ensure at
      compile time that every possible case is handled.

## Dropping a Vector Drops its Elements

```rust
{
  let v = vec![1, 2, 3, 4];

  // do stuff with v
} // <- v goes out of scope and is freed here
```

- When the vector gets dropped, all of its contents are also dropped, meaning
  the integers it holds will be cleaned up. The borrow checker ensures that any
  references to contents of a vector are only used while the vector itself is
  valid.
