---
id: nestingLoops
aliases:
  - Nesting Loops
tags:
  - bash
  - loops
---

# Nesting Loops

```bash
#!/usr/bin/bash
# Nesting Loops

for (( a = 1; a <= 3; a++ ))
do
    echo "Starting loop $a:"
    for (( b = 1; b <= 3; b++ ))
    do
        echo "Inside loop: $b"
    done
done
```

The nested loop (also called _inner loop_) iterates through its values for each
iteration of the outer loop. Notice that there's no difference between the `do`
and `done` commands for the two loops. The bash shell knows that the first `done` command is executed refers to the inner loop not the outer.

The same applies when you mix loop commands, such as placing a `for` loop inside
a `while` loop:

```bash
#!/usr/bin/bash
# placing a for loop inside a while loop

var1=5

while [ $var1 -ge 0 ]
do
  echo "Outer loop: $var1"
  for (( var2 = 1; var2 <= 3; var2++ ))
  do
    var3=$[ $var1 * $var2 ]
    echo " Inner loop: $var1 * $var2 = $var3"
  done
  var1=$[ $var1 - 1 ]
done
```

Output:

```bash
Outer loop: 5
Inner loop: 5 * 1 = 5
Inner loop: 5 * 2 = 10
Outer loop: 4
Inner loop: 4 * 1 = 4
Inner loop: 4 * 2 = 8
Outer loop: 3
Inner loop: 3 * 1 = 3
Inner loop: 3 * 2 = 6
Outer loop: 2
Inner loop: 2 * 1 = 2
Inner loop: 2 * 2 = 4
Outer loop: 1
Inner loop: 1 * 1 = 1
Inner loop: 1 * 2 = 2
Outer loop: 0
Inner loop: 0 * 1 = 0
Inner loop: 0 * 2 = 0
```

Again, the shell distinguished between `do` and `done` commands of the inner
`for` loop from the same commands in the outer `while` loop.
