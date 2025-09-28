---
id: Referencing struct fields
aliases:
  - Referencing struct fields
tags: []
---

# Referencing struct fields

Recall from your calculator tests that to refer to one of the fields on the tc
variable representing the test case, such as want, we used this dot notation:

```go
tc.want
```

When we come to write our test for the “buying a book reduces the number of
copies available” behaviour, we’ll need to refer to fields on our Book struct
too. For example, if we have some b variable representing a book, we could get
the value of its `Copies` field using dot notation:

```go
b.Copies
```

## Writing the test

We need to write a test for the "buy a book" story.. With a `want` and a `got`

We'll write a function that takes some Book value and returns a modified Book
value and decrease the number of copies of the book by 1.

Suppose we did something like this: call the `Buy` function with some book that
has a non‐zero number of copies, and then check the result to make sure it has
one less copy than we started with.

GOAL: Write `TestBuy` along these lines. Don’t worry about making it pass yet;
just write the complete test function that expresses the logic we just outlined.
It should fail to compile because the function you’re calling isn’t defined;
that’s okay.

```go
func TestBuy(t *testing.T) {
  t.Parallel()
  b := bookstore.Book{
    Title: "Spark Joy",
    Author: "Marie Kondo",
    Copies: 2,
  }
  want := 1
  result := bookstore.Buy(b)
  got := result.Copies
  if want != got {
    t.Errorf("want %d, got %d", want, got)
  }
}
```

- Based on our plan, we first set up our `b` and `want` variables, then call the
  `Buy` function, and check that we got the expected number of copies remaining.

- If not, we fail the test with an informative error message. It's not enough
  to say `failed`; what failed?

- We can’t just say unexpected number of copies, either; in
  order to fix the problem, we’d need to know what the actual number of copies was.
  It’s not even really sufficient to say want X copies, got Y; we need to make it
  clear that if we started with 2 copies and bought one, we should have one left,
  but we got some other number.

- Maybe this seems excessive, but you’ll appreciate this level of detail one day
  when you suddenly get a failing test for no apparent reason and everyone around
  you is giving you meaningful looks, wondering why they can’t ship. When tests
  fail, they should immediately tell you what to fix, in the clearest and most
  explicit way possible.

  [[Getting to a failing test.md]]
