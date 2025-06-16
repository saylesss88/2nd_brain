---
id: Variables
aliases:
  - Variables
tags: []
---

# Variables

A variable is a storage location, with a specific type and an associated name.

```go
package main

import "fmt"

func main() {
    var x string = "Hello, World"
    fmt.Println(x)
}
```

Here rather than send Hello, World directly to the `Println` function, we assign
it to a variable instead.

Variables in Go are created by first using the var keyword, then specifying
the variable name (x) and the type (string), and finally, assigning a value to
the variable (Hello, World).

Assigning a value is optional, so we could use two statements, like this:

```go
package main

import "fmt"

func main() {
  var x string
  x = "Hello, World"
  fmt.Println(x)
}
```

First, when we see the = symbol, we have a tendency to read that as “x equals
the string Hello, World.” There’s nothing wrong with reading our program that way,
but it’s better to read it as “x takes the string Hello, World” or “x is assigned the
string Hello, World.” This distinction is important because (as their name would
suggest) variables can change their value throughout the lifetime of a program. Try
running the following:

```go
package main

import "fmt"

func main() {
    var x string
    x = "first"
    fmt.Println(x)
    x = "second"
    fmt.Println(x)
}
```
