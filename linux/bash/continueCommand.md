---
id: continueCommand
aliases:
  - Continue Command
tags:
  - bash
  - commands
---

# Continue Command

The `continue` command is a way to prematurely stop processing commands inside
of a loop but not terminate the loop completely.

```bash
#!/usr/bin/env bash
# continue command

for (( var1 = 1; var1 < 15; var1++ )); do
  if [ $var1 -gt 5 ] && [ $var1 -lt 10 ]; then
    continue
  fi
  echo "Iteration number: $var1"
done
```

If the conditions of the `if-then` statement are met, the shell executes the
`continue` command, which skips the rest of the commands in the loop, but keeps
the loop going. When the `if-then` condition is no longer met, things return to
normal.

As with the `break` command, the `continue` command allows you to specify what
level of loop you want to continue with a command line parameter:

`continue n`

where `n` defines the loop level to continue:

```bash
# continuing an outer loop
for (( a = 1; a <= 5; a++ )); do
  echo "Iteration $a:"
  for (( b = 1; b < 3; b++ )); do
    if [ $a -gt 2 ] && [ $a -lt 4 ]; then
      continue 2
    fi
    var3=$[ $a * $b ]
    echo "The result of $a * $b is $var3"
  done
done
```

- The `if-then` statement uses the `continue` command to stop processing the
  commands inside the loop but continue the outer loop.

## Processing the Output of a Loop

You can either pipe or redirect the output of a loop within your shell script.
You do this by adding the processing command to the end of the `done` command:

```bash
for file in /home/jr/*; do
  if [ -d "$file" ]; then
    echo "$file is a directory"
  else
    echo "$file is a file or a special file"
  fi
done > output.txt
```
