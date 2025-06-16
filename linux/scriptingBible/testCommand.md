---
id: testCommand
aliases:
  - Trying the test Command
tags:
  - bash commands
---

# Trying the test Command

The bash if-then statement can only evaluate the commands exit status code, not
any other conditions.

The `test` command provides a way to test different conditions in an `if-then`
statement.

- If the condition listed in the `test` command evaluates to `TRUE`, the `test`
  command exits with a zero exit status code. This makes the `if-then` statement
  behave in much the same way that they work in other programming languages.

- If the condition is `FALSE`, the `test` command exits with a non-zero exit
  status code, which causes the `if-then` statement to exit.

The format of the `test` command:

```bash
test condition
```

The condition is a series of parameters and values that the `test` command
evaluates. When used in an `if-then` statement, the `test` command looks like
this:

```bash
if test condition
then
    commands
fi
```

The bash shell provides an alternative way of testing a condition without
declaring the `test` command in the `if-then` statement:

```bash
if [ condition ]
then
    commands
fi
```

- The square brackets define the test condition. Be careful; you must have a
  space after the first bracket and a space before the last bracket, or you'll get
  an error message.

If you leave out the `condition` protion of the `test` command statement, it
exits with a non-zero exit status code and triggers any `else` block statements:

```bash
#!/usr/bin/bash test6.sh
# Testing the test command
#
if test
then
    echo "No expression returns a True"
else
    echo "No expression returns a False"
fi

test6.sh
No expression returns a False
```

Determine if a variable has content:

```bash
#!/usr/bin/bash test6.sh
# Testing the test command
#
my_variable="Full"
#
if test $my_variable
then
    echo "The $my_variable expression returns a True"
#
else
    echo " The $my_variable expression returns a False"
fi
test6.sh
The my_variable expression returns a True
```

- The variable `my_variable` contains content (`Full`), so when the `test`
  command checks the condition, the exit status returns a zero. This triggers the
  statement in the `then` block.

[[numericComparisons.md]]
