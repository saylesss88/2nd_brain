---
id: Zero values & default values
aliases:
  - Zero values & default values
tags: []
---

# Zero values & default values

You might have wondered what happens if you declare a new variable using the
`var` keyword, but then don't assign it any value. What heppens when you try
to print the variable?:

```go
var x int
fmt.Println(x)
// 0
```

It turns out the variable has a default value, which is what it contains
before we explicitly put anything in it. What that value actually is depends on
the type. Each type in Go has a zero value, which is the default value for
variables of that type. For int (and float64, or any numeric type), the zero
value is zero, which makes sense, but what about other types?

  The zero value for string is the empty string, "", which also seems logical. Speaking
  of logic, what about bool variables? They default to false.

  When we declare a variable without assigning it a value, we would like its
  starting value to be zero,then we declare it with `var`:

```go
var x int
```

On the other hand, when we want the starting value to be something else, we
tend to use the short declaration syntax `:=` to both declare and assign:

```go
x := 1
```

If you’ve ever wondered when to use var and when to use :=, then this is one
way to e. If you want the default value, use var. If you want some other
value, assign it with :=.
