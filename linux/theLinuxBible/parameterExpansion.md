---
id: parameterExpansion
aliases:
  - parameterExpansion
tags: []
---

# parameterExpansion

If you want the value of a variable, you precede it with a `$` (for example,
`$CITY`).

- This is really shorthand for the notation `${CITY}`; curly braces are used
  when the value of the parameter needs to be placed next to other text without
  a space.

This list presents some common constructs you're likely to see:

`${var:-value}`: If variable is unset or empty, expand this to `value`.

`${var#pattern}`: Chop the shortest match for `pattern` from the front of
`var`'s value.

`${var##pattern}`: Chop the longest match for `pattern` from the front of var's
value.

`${var%pattern}`: Chop the shortest match for `pattern` from the end of var's
value.

`${var%%pattern}`: Chop the longest match for `pattern` from the end of var's
value.

```bash
$ THIS="Example"
$ THIS=${THIS:-"Not Set"}
$ THAT=${THAT:-"Not Set"}
$ echo $THIS
Example
$ echo $THAT
Not Set
```

Three commands that follow would all store the value 64 in the RESULT variable. The bc
command is a calculator application that is available in most Linux distributions. The last
command gets a random number between 0 and 10 and echoes the results back to
you.

- Integer arithmetic can be performed using the built-in `let` command or
  through the external `expr` or `bc` commands.

```bash
BIGNUM=1024
let RESULT=$BIGNUM/16
RESULT=`expr $BIGNUM / 16`
RESULT=`echo "$BIGNUM / 16" | bc`
let foo=$RANDOM; echo $foo
```

Another way to grow a variable incrementally is to use $(()) notation with ++I added to
increment the value of I. Try typing the following:

```bash
$ I=0
$ echo "The value of I after increment is $((++I))"
The value of I after increment is 1
7
$ echo "The value of I before and after increment is $((I++)) and $I"
The value of I before and after increment is 1 and 2
```
