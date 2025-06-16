# Create a Go module

#go

1. `mkdir greetings`

2. `cd greetings`

3. `go mod init example.com/greetings
go: creating new go.mod: module example.com/greeting`

4. Add the following code to `greetings.go`

```go
package greetings

import "fmt"

// Hello returns a greeting for the named person.
func Hello(name string) string {
    // Return a greeting that embeds the name in a message.
    message := fmt.Sprintf("Hi, %v. Welcome!", name)
    return message
}
```

In this code you: - Declare a greetings package to collect related functions. - Implement a Hello function to return a greeting.
