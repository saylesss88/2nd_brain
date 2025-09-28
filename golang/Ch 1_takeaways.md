---
id: Ch.1 Takeaways
aliases:
  - Ch.1 Takeaways
tags: []
---

# Ch.1 Takeaways

• The `go mod init` command initialises a new Go project folder.

• The `go test` command runs tests for a Go package.

• The `gofmt` command will check Go code for standard formatting and fix it if ne‐
cessary.

• A *function declaration* announces the name of a new function, and lists the parameters it takes, along with any results it returns.

• Whenever a Go test fails, it shows the exact file and line where the failure happened, as well as what failed.

• The `import` keyword tells Go what other packages we want to use in this particular Go file.

• The testing package is part of Go’s standard library, and it provides everything we need to write and run tests.

• The names of test functions begin with the word `Test`, and they take a *testing.T parameter which lets us control the test execution.

• Use the *want and got* pattern to structure tests: express what result you want, and then compare it with what you got from calling the function under test.

• We can call `t.Errorf` to cause the test to fail with a message, and if we don’t do
that, the test will pass by default.

- An if statement is used to take some action (for example, fail the test)if some condition is true.
