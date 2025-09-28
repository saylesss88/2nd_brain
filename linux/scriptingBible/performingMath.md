---
id: performingMath
aliases:
  - Performing Math
tags: []
---

# Performing Math

The `expr` command

Originally,the Bourne shell provided a special command called `expr` to perform
arithmetic. This command is now obsolete.

Although the standard operators work ﬁ ne in the expr command, the problem occurs when
using them from a script or the command line. Many of the expr command operators have
other meanings in the shell (such as the asterisk).

```bash
expr 5 * 2
expr: syntax error
```

To solve this, you need to use the shell excape character `\` before the
asterisk.

```bash
expr 5 \* 2
10
```

## Using brackets

The bash shell includes the `expr` command to stay compatible with the Bourne
shell; however, it also provides a much easier way to perform arithmetic. In
bash, when assigning a mathematical value to a variable, you can enclose the
mathematical equation using a dollar sign and square brackets.(`$[ operation ]`)

```bash
var1=$[1 + 5]
echo $var1
6
var2=$[$var1 * 2]
echo $var2
12
```

The same technique also works in shell scripts:

```bash
$ cat test7
#!/bin/bash
var1=100
var2=50
var3=45
var4=$[$var1 * ($var2 - $var3)]
echo The final result is $var4
```

The output is:

```bash
$ chmod u+x test7
$ ./test7
The final result is 500
```

The bash shell mathematical operators only support integer arithmetic, consider
using zsh if you require floating point arithmetic.

### A floating-point solution

There are several solutions, the most popular one is using the `bc` command.
(bash calculator)

**The basics of bc**

The bash calculator is actually a programming language that allows you to enter
floating-point expressions at a command line and then interprets the
expressions, calculates them, and returns the result.

The bash calculator recognizes the following:

- Numbers (int and floats)

- Variables (simple and arrays)

- Comments (C style /\* \*/)

- Programming statements (such as if-then statements)

- Functions

To use the bash calculator, install the `bc` package and type `bc` and start
entering the expression.

- Floating-point arithmetic is controlled by a built-in variable called `scale`.
  You must set this value to the desired number of decimal places you want in
  your answers, or you won't get what you're looking for:

```bash
bc -q
3.44 / 5
0
scale=4
3.44 / 5
.6880
quit
```

- The default value for `scale` is 0.

`bc` also understands variables:

```bash
bc -q
var1=10
var1 * 4
40
var2 = var1 / 5
print var2
2
quit
```

### Using `bc` in scripts

You can use the command substitution character to run a `bc` command and assign
the output to a variable! The basic format to use is this:

```bash
variable=$(echo "options; expression" | bc)
```

- The first portion, `options`, allows you to set variables.

- The `expression` parameter defines the mathematical expression to perform with
  `bc`.

```bash
#!/usr/bin/bash
var1=$(echo "scale=4; 3.44 / 5" | bc)
echo The answer is $var1
The answer is .6880
```
