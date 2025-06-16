---
id: controllingTheLoop
aliases:
  - Controlling the loop
tags:
  - bash
  - loops
---

# Controlling the loop

A couple commands help control what happens inside of a loop:

- The `break` command

- The `continue` command

## The break command

**Breaking out of a single loop**

When the shell executes a `break` command, it attempts to break out of the loop
that's currently processing:

```bash
#!/usr/bin/bash
# breaking out of a for loop

for var1 in 1 2 3 4 5 6 7 8 9 10
do
  if [ $var1 -eq 5 ]
  then
    break
  fi
  echo "Iteration number: $var1"
done
echo "The for loop is completed."
```

This works for `while` and `for` loops:

```bash
#!/usr/bin/bash
# breaking out of a while loop

var1=1

while [ $var1 -lt 10 ]; do
  if [ $var1 -eq 5 ]
  then
    break
  fi
  echo "Iteration: $var1"
  var1=$[ $var1 + 1 ]
done
echo "The while loop is completed."
```

```bash
Iteration: 1
Iteration: 2
Iteration: 3
Iteration: 4
The while loop is completed.
```

- The `while` loop terminated when the `if-then` condition was met, executing
  the `break` command.
