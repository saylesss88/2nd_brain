---
id: Comparing floating-point values
aliases:
  - Comparing floating-point values
tags: []
---

# Comparing floating-point values

Floating-point numbers have limited precision.

If you think about the decimal representation of a fraction like one‐third, it’s
0.33333333..., but it never stops. It’s threes all the way down. Representing a
number like this in a fixed number of bits, is bound to lose precision.

Let's try adding a case like this to our existing `TestDivide` cases:

```go
{a: 1, b: 3, want: 0.333333,},
```

This fails with the puzling result:

```go
Divide(1.000000, 3.000000): want 0.333333, got 0.333333
```

Instead, what we can do is add a function to do a looser comparison: not equal, exactly,
but just “close enough”, for some value of “enough”.
Let’s put this in the calculator_test package, which is where we’ll need to use it:

```go
func closeEnough(a, b, tolerance float64) bool {
return math.Abs(a-b) <= tolerance
}
```

Then we compare our `tc.want` and `got` values using this function instead of `==`:

```go
if !closeEnough(tc.want, got, 0.001) {
```

The third argument specifies how far apart the values are allowed to be and still be
considered “equal” for our purposes. In this case, that gap is 0.001. We may need to
tune this value depending on the test cases, or the requirements for the specific pro‐
gram we’re working on. For example, a navigation program for a spaceship may re‐
quire fewer decimal places of accuracy than a robotic brain surgery system.
