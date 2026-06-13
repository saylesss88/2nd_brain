# Slices

Slices are dynamically sized array-like objects. The term _dynamically sized_
means their size is not known at compile time. Yet, like arrays, these don't
expand or contract. The lack of compile-time knowledge explains the distinction
in their type signature betweemn an array (`[T; n]`) and a slice (`[T]`).

Slices are important because it's easier to implement traits for slices than
arrays. Traits are how Rust programmers add methods to objects.
