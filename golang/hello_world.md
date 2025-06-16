# Golang Hello World

#go

Open a command prompt and cd to your home directory.

On Linux or Mac:

1. `cd`

2. `mkdir hello`

3. Enable dependency tracking for your code
   `go mod init example/hello`

4. Create file hello.go
   `vi hello.go`

5. hello.go contents

```go
package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}
```

- Declare a main package (a package is a way to group functions, and it's made up of all the files in the same directory).

- Import the popular fmt package, which contains functions for formatting text, including printing to the console. This package is one of the standard library packages you got when you installed Go.

- Implement a main function to print a message to the console. A main function executes by default when you run the main package.

6. Run your code to see the greeting.
   `go run .`
   Hello, World!

`go help`
