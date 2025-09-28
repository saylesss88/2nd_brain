# go_switch
#go

The 'switch' control flow works much the same as using 'if''else', or 'else if' but presents differently in the form 
of 'cases'.
- [>] A 'case' is the same as a condition and may be preferred by those with a mathematical background.
- [~] Most explicit way to share what the code is training to do.

```go
// ex.4 switch
package main

import (
    "fmt"
    "time"
)

func main() {
    // 'switch' without an expression is an alternate way
    // to express if/else logic
    t := time.Now()
    switch {
    case t.Hour() < 12:
        fmt.Println("It's before noon")
    default:
        fmt.Println("It's after noon")
}

}
```
In the above, we say 'switch' based upon the following 'case' and pass in some data.
The same can be accomplished by using 'if' and 'else if'

```go
// ex. 5 convert switch to if else
package main

import (
	"fmt"
	"time"
)

func main() {
    t := time.Now()
	if t.Hour() < 12 {
		fmt.Println("before noon")
	} else if t.Hour() > 12 {
		fmt.Println("it's after noon")
	}
}

```
