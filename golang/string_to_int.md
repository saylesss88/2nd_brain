---
id: string_to_int
aliases:
  - golang string to int
tags: []
---

# golang string to int
#go

```go
package main

import (
    "fmt"
    "reflect"
    "strconv"
)

func main() {
    strVar := "100"
    intVar, err := strconv.Atoi(strVar)
    fmt.Println(intVar, err, reflect.TypeOf(intVar))
}
```
