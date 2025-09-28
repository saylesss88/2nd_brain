# Exported Identifiers

The type `Customer` has a capital `C`

- Names with an initial capital letter have a special meaning in Go.
- Anything with such a name is available outside the package where it's defined
  (we say it's exported).
- If you define a type (or a function, a variable, or anything else) whose name
  starts with a lowercase letter (`customer`, for example), it is unexported
  and so you won't be able to refer to it in code that's outside this package
  (in a test, for example)

  Think of exported names, with the initial capital letter, as identifying things
  that the package intends to be _public_, while the unexported names, with an
  initial lowercase letter, are _private_ that the package is going to use
  internally, but no one else needs to know about it.

[[The core package.md]]
