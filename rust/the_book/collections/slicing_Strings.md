---
id: slicing_Strings
aliases:
    - slicing_Strings
tags: []
---

# slicing_Strings

Indexing into a string is often a bad idea because it's not clear what the
return type of the string-indexing operation should be: a byte value, a
character, a grapheme cluster, or a string slice.

Rather than index using `[]` with a single number, you can use `[]` with a range
to create a string slice containing particular bytes:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

- Here, `s` will be a `&str` that contains the first four bytes of the string.
  Each of these characters are two bytes, which means `s` will be `Зд`.
