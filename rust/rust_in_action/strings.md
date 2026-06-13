# Strings

`String` and `&str` both represent text, yet are distinct types.

`String`'s support familiar operations like concatenation, appending new text
onto an existing string, or trimming whitespace.

`str` is a high-performance, relatively feature-poor type. Once created, `str`
values cannot expand or shrink.

`str` is usually seen in the form `&str`. A `&str` (string slice) is a small
type that contains a reference to `str` data and a length. `&str` is a borrowed type, you can think of it as read-only data, whereas `String` is
read-write.

String literals have the type `&str`. 
