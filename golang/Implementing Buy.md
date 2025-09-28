# Implementing Buy

**GOAL**: Implement the `Buy` function, so that it passes the test.

```go
func Buy(b Book) Book {
  b.Copies--
  return b
}
```

- A test needs to _describe_ the entire behavior required, whereas the function
  only has to _implement_ the behavior.

- If about half of your codebase consists of tests, overall, that's probably
  right. More is fine. Less might indicate that you're missing a few tests.

  [[Test coverage.md]]
