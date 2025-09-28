---
id: Type definitions
aliases:
  - Type definitions
tags: []
---

# Type definitions

We begin with a comment, introduced by the // characters. It’s good practice to add
an explanatory comment for each type you create in Go, telling readers what the type
is intended to do. These are called documentation comments, because when you pub‐
lish your project to a hosting site such as GitHub, your comments will be automatic‐
ally turned into browsable documentation on the pkg.go.dev site.

Next, we can see a new keyword here: `type` - This introduces a new _type definition_ - With the `type` keyword, we're saying "Please create a type called ..."
and here are the details... - The details, in this case, are the keyword `struct`, and a list of fields
inside curly braces.

Everything after `Customer` is called a _type literal_

[[Exported Identifiers.md]]
