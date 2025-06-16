---
id: Variables and values
aliases:
  - Variables and values
tags: []
---

# Variables and values

```go
package main
import "fmt"
func main() {
    var title string
    var copies int
    title = "For the Love of Go"
    copies = 99
    fmt.Println(title)
    fmt.Println(copies)
}
```

Let's break down whats going on here. As we learned earlier, all executable Go
programs must have a package `main`. They also have a function `main` that is
called automatically when the program runs.

  First, we *declare* a variable title, of type string:

```go
var title string
```

  The var keyword means "Please create a variable called... of type..." In this
  case, please create a variable called `title` of type `string`.

  Next, we declare a variable copies of type int (a numeric type for storing
  integers, that is, whole numbers).

```go
var copies int
```

  You can read this as "The variable `title` is assigned the value `For the Love
  of Go`" to the `title` variable", using the `=` operator.

```go
title = "For the Love of Go"
```

  This value, is a "string literal"(It's literally this string!)

  Next we assign the value `99` to the `copies` variable.

```go
copies = 99
```

Finally, we use the `fmt.Println` function to print the value of `title` and
`copies`:

```go
fmt.Println(title)
fmt.Println(copies)
```
