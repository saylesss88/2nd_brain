# Assigning to struct fields

Suppose that inside the Buy function, we have our b parameter, which we know is
a Book, and we need to update it to reflect the fact that there is one less copy
left in stock. How can we do that?

- To assign a value to `Copies`:

```go
b.Copies = 7
```

- To decrement the value of `Copies`:

```go
b.Copies--
```

- To increment the value of `Copies`:

```go
b.Copies++
```

- Incidentally, if we want to adjust
  some variable’s value by more than 1, we can use this form:

```go
b.Copies -= 5
```

This subtracts 5 from the value of `b.Copies`

[[Implementing Buy.md]]
