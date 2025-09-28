---
id: Type checking
aliases:
  - Type checking
tags: []
---

# Type checking

We said that the Go compiler will use the type information you’ve provided to
check whether you accidentally assigned a value to the wrong type of variable.
What does that look like? Let’s try something silly:

```go
title = copies
```

We know this won't work right away, `title` is a string variable, so it can only
hold string values.
  But `copies` is an int variable, so it can only hold int values.

  If you add this line to the program, what happens? We get
  an error message from the compiler:

```go
cannot use copies (type int) as type string in assignment
```

That makes total sense, given what we know. Another way to phrase this would be
“You tried to assign a value of type int to a string variable, and that’s not
allowed”

**Type checking** happens in other places too, such as when passing parameters
to a function.

  Suppose we declare a function `printTitle`, which takes a string parameter:

```go
func printTitle(title string) {
    fmt.Println("Title: ", title)
}
```

When we call this function, we need to pass it a value of the specified type.
So title would be okay here, because it’s of type string, and so is the functi-
on’s parameter:

```go
printTitle(title)
```

You already know that trying to pass it an int value won’t work, but let’s try
it:

```go
printTitle(copies)
```

Just as we thought, that's a *type* error:
`cannot use copies (type int) as type string in argument to
printTitle`
