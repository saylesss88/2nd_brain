---
id: Lifetime Annotation Syntax
aliases: [lifetimes]
---

# Lifetime Annotation Syntax

Lifetime annotations describe the relationship of the lifetimes of multiple references to each other without affecting the lifetimes.

- Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.

- Lifetime annotations syntax: the names of lifetime parameters must start with an apostrophe (`'`) and are usually all lowercase and very short. Typically the first is `'a` next is `'b` and so on.
- We place lifetime parameter annotations after the `&` of a reference, using a space to separate the annotation from the reference type.

```rust
&i32         // a reference
&'a i32      // a reference with an explicit lifetime
&'a mut i32  // a mutable reference with an explicit lifetime
```

- One lifetime annotation by itself doesn't have much meaning because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other.

## Lifetime Annotations in Function Signatures

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
```
- The `longest` function definition specifying that all the references in the signature must have the same lifetime `'a`.

