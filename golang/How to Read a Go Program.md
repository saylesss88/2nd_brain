---
id: How to Read a Go Program
aliases:
  - How to Read a Go Program
tags: []
---

# How to Read a Go Program

```go
package main

import "fmt"

func main() {
  fmt.Println("Hello, World!")
}
```

Go programs are read top to bottom, left to right (like a book) The first line
says:

```go
package main
```

This is known as a _package declaration_, and every Go program must start with
it.

On the following line we see:

```go
import "fmt"
```

- The `import` keyword is how we include code from other packages to use in our
  program.
- The `fmt` package implements formatting for input and output.
- Notice that `fmt` is surrounded by double quotes. The use of double quotes
  like this is known as a _string literal_, which is a type of expression.

Then we see:

```go
func main() {
  fmt.Println("Hello, World!")
}
```

Functions are the building blocks of a Go program. They have inputs, outputs,
and a series of steps called statements that are executed in order.

- All functions start with the `func` keyword, followed by the name of the
  function. (main in this case)
- A list of zero or more _parameters_ is enclosed in parentheses.
- An optional _return type_ is enclosed in square brackets.
- And a body which is surrounded by curly braces.

The name `main` is special because it's the function that gets called when you
execute the program.

The final piece of our program is the line:

```go
fmt.Println("Hello, World!")
```

This statement is made of three components. First, we access another function
inside of the `fmt` package: `Println`.

Then we create a new string that contains the string `Hello, World!`.and invoke
(or execute) that function with the string as the first and only argument.

The Println function does the real work in this program. You can find out more
about it by typing the following in your terminal:

```bash
go doc fmt Println
```

package fmt // import "fmt"

```bash OUTPUT:
func Println(a ...any) (n int, err error)
    Println formats using the default formats for its operands and writes to
    standard output. Spaces are always added between operands and a newline
    is appended. It returns the number of bytes written and any write error
    encountered.
```

Go is a very well-documented programming language, but this documentation can be
difficult to understand unless you are already familiar with programming languages.
Nevertheless, the godoc command is extremely useful and a good place to start when‐
ever you have a question.

[[chapter2.md]]
