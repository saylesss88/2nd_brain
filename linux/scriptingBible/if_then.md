---
id: if_then
aliases:
  - Working with the if-then Statement
tags:
  - bash
  - flow
---

# Working with the if-then Statement

The most basic type of structured command is the `if-then` statement:

Syntax:

```bash
if command
then
    commands
fi
```

- if-then statements in other programming languages, the object after the `if`
  statement is an equation that is evaluated to TRUE or FALSE. Thats not how the
  bash shell if statement works.

- The bash shell if statement runs the command defined on the if line. If the
  exit status of the command is zero, the commmands listed under the `then`
  section are executed.

- If the exit status is anything else, the `then` commands aren't executed, and
  the bash shell moves on to the next command in the script.

  - The `fi` statement delineates the if-then statement's end.

```bash
#!/usr/bin/bash
# testing the if statement
if pwd
then
    echo "it worked"
fi
```

If the `pwd` command completes successfully, the `echo` statement should display
the text string.

```bash
./test1.sh
/home/jr
it worked
```

- The shell executed the `pwd` command listed on the `if` line. Because the exit
  status was zero, it also executed the `echo` statement listed in the `then`
  section.

Another example:

```bash
#!/bin/bash
# testing a bad command
if IamNotaCommand
then
echo "It worked"
fi
echo "We are outside the if statement"
$
$ ./test2.sh
./test2.sh: line 3: IamNotaCommand: command not found
We are outside the if statement
```

An alternative form of the `if-then` statement used in some scripts:

```bash
if command; then
    commands
fi
```

- By putting the semicolon at the end of the command to evaluate, you can
  include the `then` statement on the same line, which looks closer to how other
  languages handle if-then statements.
