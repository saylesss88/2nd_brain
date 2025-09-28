# Testing the core struct type

- A struct type doesn't really have any behavior itself, but we'd like to at least
  establish the core `Book` type before we move on.

- So lets write a simple test that doesnt actually do anything, but can't pass
  unless the `Book` type has been identified.

- That would mean that we need to use a value of that type. What would that look
  like?

## Struct literals

We've seen some literals of basic types already.

There are struct literals too. They look like this:

```go
bookstore.Book{
  Title: "Nicholas Chuckleby",
  Author: "Charles Dickens",
  Copies: 8,
}
```

The literal begins with the name of the type (`Book`), followed by curly braces
Then we write inside the curly braces a list of the pairs of field names and
values (for example, `Title: "Nicholas Chuckleby"`).

### Assigning a struct literal

What should we do with our type `Book`? Here's one idea:

```go
func TestBook(t *testing.T) {
  t.Parallel()
  _ = bookstore.Book{
    Title: "Spark Joy",
    Author: "Marie Kondo",
    Copies: 2,
  }
}
```

It seems like this isn’t really testing anything, because there’s no way for it to fail.
There’s no call to `t.Error`, for example, or even any if statement that could decide
whether the test passes or fails. It should always pass.

So what's actually happening in this test? There's some struct literal of type
`Book`, and we're assigning it to something. To what?
