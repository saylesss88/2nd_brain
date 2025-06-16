---
id: Strings
aliases:
  - Strings
tags: []
---

# Strings

- [!] String literals can be created using double quotes or backticks. The
  difference is that double-quoted strings cannot contain newlines and they allow
  special escape sequences.

Some common operations on strings:

```go
len("Hello, World")
```

```go
"Hello, World"[1]
    Access a particular character in a string
```

```go
"Hello, " + World"
   Concatenate strings
```

Let's try some examples:

```go
package main

import "fmt"

func main() {
    fmt.Println(len("Hello, World"))
    fmt.Println("Hello, World"[1])
    fmt.Println("Hello, " + "World")
}
```

OUTPUT:

```go
12
101
Hello, World
```

A few things to notice:
• A space is also considered a character, so the string’s length is 12, not 11, and the
third line has "Hello, " instead of "Hello".
• Strings are indexed starting at 0, not 1. [1] gives you the second element, not the
first. Also notice that you see 101 instead of e when you run this program. This is
because the character is represented by a byte (remember a byte is an integer).
One way to think about indexing would be to show it like this instead: "Hello,
World"1. You’d read that as “The string Hello, World sub 1,” “The string Hello,
World at 1,” or “The second character of the string Hello, World.”
• Concatenation uses the same symbol as addition. The Go compiler figures out
what to do based on the types of the arguments. Because both sides of the + are
strings, the compiler assumes you mean concatenation and not addition (addi‐
tion is meaningless for strings).

[[Booleans.md]]
