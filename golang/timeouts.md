---
id: timeouts
aliases:
  - timeouts
tags: []
---

# timeouts

- Timeouts are important for programs that connect to external resources or that otherwise need to bound execution time.

- Implementing timeouts in Go is easy and elegant thanks to channels and select.

```go
package main

import (
    "fmt"
    "time"
)
```
