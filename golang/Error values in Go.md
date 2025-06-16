---
id: Error values in Go
aliases:
  - Error values in Go
tags: []
---

# Error values in Go

There’s something about the Divide function that wasn’t true of Add, Subtract,
and Multiply: it’s possible to give it invalid input. So what value should we return in that case?

A cool thing about functions in Go is that they're not limited to returning a single value.
They can return multiple values and often do.

It sounds like Divide should return two values. One will be the result of the calcula‐
tion, as with the other functions, but the other can be an error indicator. If something
went wrong, like attempting to divide by zero, the error value can convey that informa‐
tion back to the code that called Divide.

Go has a special type called `error`. It's very common for Go functions to return two values,
the first being the result of the calculation, and the second being an error indicator.
