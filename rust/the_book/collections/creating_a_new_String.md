---
id: creating_a_new_String
aliases: []
tags: []
---

# Creating a new String

Many of the same operations available with `Vec<T>` are available with `String`
as well because `String` is actually implemented as a wrapper around a vector of
bytes with some extra guarantees, restrictions, and capabilities.

A function that works the same way with `Vec<T>` and `String` is the `new`
function to create an instance:

```rust
let mut s = String::new();
```

- This creates a new empty string called `s`, into which we can then load data.

- Often, we'll have some initial data with which we want to start the string.
  For that, we use the `to_string` method, which is available on any type that
  implements the `Display` trait, as string literals do.

```rust
let data = "initial contents";

let s = data.to_string();

let s = "initial contents".to_string();
```

We can also use the function String::from to create a String from a string literal.

```rust
let s = String::from("initial contents");
```

- Using the `String::from` function to create a `String` from a string literal

- `String::from` and `to_string` do the same thing.

## Updating a String

- A `String` can grow in size and its contents can change. You can also use the
  `+` operator or the `format!` macro to concatenate `String` values.

We can grow a `String` by using the `push_str` method to append a string slice:

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

- After these 2 lines, `s` will contain `foobar`.

- The `push_str` method takes a string slice because we don't necessarily want
  to take ownership of the parameter.

Here we want to be able to use `s2` after appending its contents to `s1`:

```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {s2}");
```

- If the `push_str` method took ownership of `s2`, we wouldn't be able to print
  its value on the last line.

The `push` method takes a single character as a parameter and adds it to the
`String`:

```rust
let mut s = String::from("lo");
s.push('l');
```

- `s` will contain `lol`.

## Concatenation

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1 has been moved here and can no longer be used
```

- `s3` will contain `Hello, world!`.

- This statement actually takes ownership of `s1` and appends a copy of the
  contents of `s2`, and then returns ownership of the result.

[[internal_Representation.md]]
