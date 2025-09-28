# Assigning a struct literal

So now we know how to write a value of type `Book`, what shall we do with it?

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

It seems like this isn’t really testing anything, because there’s no way for it
to fail.There’s no call to t.Error, for example, or even any if statement that
could decide whether the test passes or fails. It should always pass.

So what's actually happening in this test? There's some struct literal of type
`Book`, and we're assigning it to something. To what?

Remember the blank identifier (`_`) we use to ignore a value, because we don't
need it? Same thing here. We don't want to do anything with this `Book` value.
We just want to mention it, to help us get our ideas straight about what the
`Book` type is and what fields it has.

For that, we need a literal, & since we can't just write a literal on its own
(that's not a valid Go statement) we need to assign it to something.

If we assigned it to some named
variable, though, the compiler would complain that we didn’t then use that
variable, so the blank identifier is our excuse here

## The unfailable test

A test like this can’t fail, so it should always pass. But does it actually
pass right now? No: not because it fails, but because it doesn’t compile. And
that makes sense, because it imports a non‐existent package bookstore, and
refers to a data type in it (bookstore.Book) that doesn’t exist either.

There's no way this test can compile correctly until those things are defined
somewhere (a compile-only test)

Before defining them, we would like to
be at the point where this test fails to compile with some message like
undefined:
`bookstore.Book`.

Let’s create a new file bookstore.go and add a package declaration:

```go
package bookstore
```

If we run `go test` now, we can see that we're at the point where we wanted to be:

```go Output
# bookstore_test [bookstore.test]
./bookstore_test.go:10:6: undefined: bookstore.Book
FAIL
bookstore [build failed]
```

We knew that the test couldn’t compile unless the Book type was defined and had
exactly the fields that the literal value in the test requires, and it isn’t and
it doesn’t… yet. But we’re very close. Over to you!

**GOAL**: Make this test pass. Hint: you’ll need to define a type in package
bookstore named Book. The compiler will tell you if you haven’t defined it quite
right. Just keep tweaking it and re‐running the test until you’ve got the struct
definition the way it needs to be.

```go
// Book represents info about a book
type Book struct {
  Title string
  Author string
  Copies int
}
```

### Takeaways

• Go has various data types for dealing with different kinds of information: string
holds text, int holds integer numbers, float64 holds fractional numbers, and
bool holds Boolean (true or false) values.
• The Go compiler checks that a variable or parameter of a given type is only ever
assigned values of that type: if it detects a mismatch, that’s a compile error.
• The var keyword introduces a variable, and specifies its type, so that we can use it
later on in the program.
• A literal is a value of some type that we specify directly in our Go code, such as the
string literal "For the Love of Go".
• Variables in Go have a default value before we assign something to them expli‐
citly: it’s whatever the zero value is for its type, such as "" for strings, 0 for num‐
bers, or false for bool.

• We often declare and assign values to variables in a single statement, using the
short declaration form, for example x := y.
• When you want to group related values together into a single variable, you can
define a struct type to represent them.
• The different fields of the struct are just like any other kind of Go variable, and
they can be of any Go type, including other structs.
• Type definitions are introduced by the keyword type, and struct types with the
additional keyword struct, followed by a list of fields with their types.
• Identifiers (that is, the names of functions, variables, or fields) with an initial cap‐
ital letter are exported: that is, they’re public and can be accessed from outside the
package where they’re defined.
• On the other hand, identifiers beginning with a lowercase letter are unexported:
they’re private to the package.
• Most Go packages are “about” one or more core struct types, in some sense:
they’re often the first things that you’ll define and test.
• A neat way to design a struct type guided by tests is to write a compile‐only test,
that simply creates a literal value of the struct: this test can’t pass (or, indeed,
compile) until you’ve correctly defined the struct type.

[[Story time.md]]
