---
id: Introducing structs
aliases:
  - Introducing structs
tags: []
---

# Introducing structs

A data type made up of several values that you can treat as a single unit is
called a `composite` type (as in, “composed of several parts”). One of the most
useful composite data types in Go is called a `struct`, short for “structured
record”.

A struct groups together related pieces of data, called fields. For example, a
struct describing a customer of our bookstore might have a field called Name for
their name(a string would make sense for this). Another field we might need is
Email, for their email address (let’s use string for this too).

Here's what a definition of a simple Customer type might look like:

```go
// Customer represents a customer of our bookstore
type Customer struct {
    Name string
    Email string
}
```

==Structs are a very convenient way to create a single value that contains
multiple pieces of data inside it.==

[[Type definitions.md]]
