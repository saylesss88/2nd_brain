---
id: Ch. 3 takeaways
aliases:
  - Ch. 3 takeaways
tags: []
---

# Ch. 3 takeaways

• A common compiler error is something like assignment mismatch: 2 vari-
  ables but X returns 1 values: this means that we tried to receive a differ‐
  ent number of results than the function actually returns.

• When a function declares multiple result values, the list of result types
  must be enclosed in parentheses: for example, (float64, error).

• Another common kind of compiler error is not enough arguments to re-
  turn, meaning that the function declares a certain number of result values, but
  there’s a return statement with a different number of arguments.

• When there’s no error, return the explicit value nil as the error result: nil
  means “no error”.

• The compiler won’t let us declare a variable that we never refer to again (it cor‐
  rectly infers that this must be a mistake).

• But sometimes we need a variable syntactically, as in the left‐hand side of an as‐
  signment statement: in this case, we can use the blank identifier _ to act as a place‐
  holder.

• The test failure for “invalid input” cases should say something like “want error for
  invalid input, got nil”, and since there’s no data in this message, we use t.Error
  instead of t.Errorf.

• When a function detects an error, it should return zero for the data value, or
  whatever the appropriate zero value is for its type, along with the error.

• if statements often use == or != to compare two values, but there are other kinds
  of comparison operators: for example, < (less than), <= (less than or equal to), >
  (greater than), >= (greater than or equal to).

• We can also join together two or more expressions using logical operators like &&
  (true if all expressions are true) and || (true if at least one expression is true).

• To negate an expression, use the “not” operator ! (true if the expression is false).

• People make a lot of fuss about testing, but it’s essentially very simple: first we
  work out what we want, and then we compare it against what we actually got.

• Our workflow can be summarised by the phrase “red, green, refactor”: write a
  failing test (red), implement the code to make it pass (green), and then refactor
  both the code and the test.

• If you can’t figure out how to test something, try testing a smaller, simpler func‐
  tion with less behaviour; similarly, if you can’t figure out how to implement some‐
  thing, use a simpler function as a stepping‐stone.

• Tests are all about comparisons, but we need to be careful when comparing
  floating‐point values, because there’s an inherent loss of precision in the way
  computers store these numbers.

• An exact comparison may not work with some values, in which case we can in‐
  stead check whether the want and got are close enough, for some definition of
  “enough”.

• The go run command both compiles and executes a program, which is useful for
  testing.
