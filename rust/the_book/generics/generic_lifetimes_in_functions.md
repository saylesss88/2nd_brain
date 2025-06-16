---
id: Lifetimes in Functions
alias: [lifetimes]
---

# Generic Lifetimes in Functions

```rust
fn main() {
  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest string is {result}");
}
```

- We want the function to take string slices, which are referencess, rather than strings, because we don't want the `longest` function to take ownership of its parameters.
