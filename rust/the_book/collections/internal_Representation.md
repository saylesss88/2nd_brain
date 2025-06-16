---
id: internal_Representation
aliases:
    - internal_Representation
tags: []
---

# internal_Representation

A `String` is a wrapper over a `Vec<u8>`. Let's look at some of our properly
encoded UTF-8 example strings:

```rust
let hello = String::from("Hola");
```

- In this case, `len` will be `4`, which means the vector storing the string
  `"Hola"` is 4 bytes long. Each of these letters takes one byte when encoded in
  UTF-8.

[[slicing_Strings.md]]
