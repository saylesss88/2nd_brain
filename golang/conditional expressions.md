---
id: conditional expressions
aliases:
  - conditional expressions
tags: []
---

# conditional expressions

The expression after the `if` keyword is what determines whether or not
the code inside the indented block will be executed.

If it’s true, then the indented statements will be executed. In this case, that’s the call to `t.Errorf` that causes the test to fail. On the other hand, if the expression evaluates to `false`, then nothing heppens;
the program just moves on to the next statement.

The expression in our example is this:

```go
want != got
```

We call this the *conditional expression* for the `if` statement.

- The indentation helps us visualize what's going on:

```go
if want != got {
 t.Errorf("want %d, got %d", want, got)
}
```
