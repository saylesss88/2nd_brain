---
id: usingDoubleParentheses
aliases:
  - Using double parentheses
tags: []
---

# Using double parentheses

The double parentheses command allows you to incorporate advanced mathematical
formulas in your comparisons.

`(( expression ))`

The expression term can be any mathematical assignment or comparison expression.

## Double Parentheses Command Symbols

`val++`: Post-increment

`val--`: Post-decrement

`++val`: Pre-increment

`--val`: Pre-decrement

`!`: Logical negation

`~`: Bitwise negation

`**`: Exponentiation

`<<`: Left bitwise shift

`>>`: Right bitwise shift

`&`: Bitwise Boolean AND

`|`: Bitwise Boolean OR

`&&`: Logical AND

`||`: Logical OR

```bash
#!/usr/bin/env bash
# using double parentheses
#
val1=10
#
if (( $val1 ** 2 > 90 ))
then
    (( val2 = $val1 ** 2 ))
    echo "The square of $val1 is $val2"
fi
```

### Using Double Brackets

The double brackets command provides advanced features for string comparison.
This is the format:

```bash
[[ expression ]]
```

- The double bracketed expression uses the standard string comparison used in
  the test evaluations. However, it provides pattern matching as well.

In pattern matching, you can define a regex that's matched against the string
value:

```bash
#!/usr/bin/env bash
# using pattern matching
#
if [[ $USER == j* ]]
then
    echo "Hello, $USER"
else
    echo "Sorry, I don't know you"
fi
```

- The (`==`) sign designate the string to the right (`j*`) as a pattern, and
  pattern matching rules are applied.

- The double bracket command matches the `$USER` environment variable to see
  whether it starts with the letter `j`, if so the comparison succeeds and the
  shell executes the `then` statement.
