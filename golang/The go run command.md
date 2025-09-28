---
id: The go run command
aliases:
  - The go run command
tags: []
---

# The go run command

Let’s write an executable program that has a `main` package and a `main` function,
then. We’ll use our calculator package to perform some calculation, and print
the result in the user’s terminal.

Create a new subfolder named `cmd` in the calculator project. Because there
could be more than one command associated with this package, let's create a
further subfolder calculator, too:

```bash
mkdir -p cmd/calculator
```

Add a file named `main.go` to the `cmd/calculator` folder:

```go
package main

import (
  "calculator"
  "fmt"
)

func main() {
  result := calculator.Add(2, 2)
  fmt.Println(result)
}
```
