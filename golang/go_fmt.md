# go_fmt
#go

With GO, there is only one option for formatting.

Imagine your extensions aren't working and your code looks like this:
```go
package main
import "fmt"
func main(){
fmt.Println("Hello, world!")
}
```
To make this code GO-compliant, you can run:
```go
go fmt main.go
```

And your code will look like this:
```go
package main
	 
import "fmt"
	 
func main() {
	fmt.Println("Hello, world!")
}
```

