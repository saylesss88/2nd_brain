---
id: Test coverage
aliases:
  - Test coverage
tags: []
---

# Test coverage

The relationship between test code and the code it tests (often called _system_
code to distinguish it from the test code)

The Go tools provide a code coverage function to check coverage.
`go test -cover`

```go
PASS
coverage: 100.0% of statements
```

That’s great, but what if not all statements were covered by tests? How could
we see what the uncovered ones were? Luckily, Go can generate a coverage profile
for us:
`go test -coverprofile=coverage.out`

We can now use this to generate an HTML report
`go tool cover -html=coverage.out`

This will open your default browser to view the generated HTML.

[[Test-last development.md]]
