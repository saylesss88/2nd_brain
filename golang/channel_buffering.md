# channel_buffering

#go

By default channels are unbuffered, meaning that they will only accept
sends (chan <-) if there is a corresponding receive (<- chan) ready to
recieve the sent value.

- Buffered channels accept a limited number of values without a
- corresponding receiver for those values.
-
- Here we make a channel of strings buffering up to 2 values.
-
- Because this channel is buffered, we can send these values into the channel without a corresponding concurrent receive.
-

```go
package main

import "fmt"

func main() {
   messages := make(chan string, 2)

   messages <- "buffered"
   messages <- "channel"

   fmt.Println(<-messages)
   fmt.Println(<-messages)
}
```

$`go run channel-buffering.go`
