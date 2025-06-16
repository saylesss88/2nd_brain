```go
// Tell me if a number is even or odd
package main

import (
	"fmt"
	"math/rand"
)

func main() {
	//use the rand library to generate a random integer.
	n := rand.Intn(100)
	// 'if' condition
	if n%2 == 0 {
		// code block that's executed if the condition is true
		fmt.Printf("Number is even: %d", n)
	} else {
	// code block that's executed if condition is false
	fmt.Printf("Number is odd: %d", n)
}
```
In this program, we're checking whether a 'random' integer number is even or not via the modulo operator `%`, which is a fancy way of saying 
'dividing with remainders'.

If you recall, an even number can be divided in two without any remainders, whereas an odd number cannot.
