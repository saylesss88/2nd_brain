---
id: whatIsAFactorial
aliases:
  - What is a Factorial?
tags:
  - bash
  - factorial
---

# What is a Factorial?

A factorial is a mathematical operation that you write like `n!`. It represents
the multiplication of all numbers between 1 and `n`.

So if you have `3!`, you'd compute `3 * 2 * 1` which is `6`.

## Definition

The factorial of a number is the multiplication of all the numbers between 1 and
the number itself. So the factorial of 2 is `2!` (= 1 \* 2)

To calculate a factorial you need to know two things:

1. `0! = 1`

2. `n! = (n - 1)! * n`

| Factorial | Multiplication        | Result |
| --------- | --------------------- | ------ |
| 0!        | 1                     | 1      |
| 1!        | 1                     | 1      |
| 2!        | 1 x 2                 | 2      |
| 3!        | 1 x 2 x 3             | 6      |
| 4!        | 1 x 2 x 3 x 4         | 24     |
| 5!        | 1 x 2 x 3 x 4 x 5     | 120    |
| 6!        | 1 x 2 x 3 x 4 x 5 x 6 | 720    |

### What is a Factorial Used For?

A factorial is the number of different permutations you can have with `n` items:
3 items can be arranged in exactly 6 different ways.(expressed as `3!`)

For example, all the arrangements you can have with three items, A, B and C:

```bash
ABC
ACB
BAC
BCA
CAB
CBA
```

And in fact, `3! = 6`
