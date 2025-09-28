---
id: a Sqrt function
aliases:
  - a Sqrt function
tags: []
---

# a Sqrt function

* TODO: Develop a Sqrt function that takes a float64 value and returns its
  square root as a float64 value.

The square root of a number is another number that,

 when multiplied by itself, equals the original number. It's essentially the opposite of squaring a number. Here's a breakdown:

    Squaring: Squaring a number means multiplying it by itself. For example, 4 squared (written as 4²) is 4 x 4, which equals 16.
    Square Root: The square root of a number, represented by the symbol √, is the value you need to multiply by itself to get the original number. So, the square root of 16 (written as √16) is 4, because 4 x 4 = 16.

Examples:

    √25 = 5 (because 5 x 5 = 25)
    √9 = 3 (because 3 x 3 = 9)
    √144 = 12 (because 12 x 12 = 144)

Not all numbers have perfect square roots:

    Some numbers, like 2 or 5, don't have perfect square roots. Their square roots are irrational numbers, meaning they cannot be expressed as a simple fraction and their decimal representation never ends or repeats.

Applications of Square Root:

Square roots have various applications in mathematics, physics, engineering, and other fields. Here are a few examples:

    Pythagorean Theorem: The Pythagorean theorem uses square roots to calculate the length of the hypotenuse of a right triangle.
    Areas and Distances: Square roots can be used to find areas of squares and rectangles with side lengths represented by square roots. They can also be used to calculate distances in geometric problems.
    Velocity and Acceleration: Square roots can be involved in calculations related to velocity and acceleration, especially when dealing with rates of change.

```go
func TestSqrt(t *testing.T) {
 t.Parallel()
 type testCase struct {
  input float64
  want  float64
 }
 testCases := []testCase{
  {input: 4, want: 2},
  {input: 2, want: 1.41421356237},
  {input: 25, want: 5},
  {input: 1.4, want: 1.2},
 }
 for _, tc := range testCases {
  got, err := calculator.Sqrt(tc.input)
  if err != nil {
   t.Fatalf("Sqrt(%f): want no error for valid input, got %v", tc.input, err)
  }
  if !closeEnough(tc.want, got, 0.1) {
   t.Errorf("Sqrt(%f): want %f, got %f",
    tc.input, tc.want, got)
  }
 }
}
```

```go
func TestSqrtInvalid(t *testing.T) {
 t.Parallel()
 _, err := calculator.Sqrt(-1)
 if err == nil {
  t.Error("Sqrt(-1): want error for invalid input, got nil")
 }
}
```

```go
func closeEnough(a, b, tolerance float64) bool {
 return math.Abs(a-b) <= tolerance
}
```
