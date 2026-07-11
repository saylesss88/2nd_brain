# HashMap

The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type
`V` using a _hashing_ function, which determines how it places these keys and
values into memory.

Hash maps are useful when you want to look up data not by using an index, as you
can with vectors, but by using a key that can be of any type.

## Creating a New Hash Map

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

This `HashMap` has keys of type `String` and values of type `i32`.


