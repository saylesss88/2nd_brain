# The short declaration form

You already know that you can declare a variable and assign it a value in one
go, using a var statement with =:

```go
var b = bookstore.Book{
    Title: "Nicholas Chuckleby",
    Author: "Charles Dickens",
    Copies: 8,
}
```

And you may also recall from the calculator example that there’s a short
declaration form that uses :=, like this:

```go
b := bookstore.Book{
    Title: "Nicholas Chuckleby",
    Author: "Charles Dickens",
    Copies: 8,
}
```

So we can create some variable b that holds a Book value. What can we do with
it? How can we access those fields like Title and Author?

[[Referencing struct fields.md]]
