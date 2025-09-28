---
id: C-style_for_loop
aliases:
  - C-style for loop
tags:
  - bash
  - loops
---

# C-style for loop

Basic format of the C-style bash `for` loop:

```bash
for (( variable assignment ; condition ; iteration process ))
```

The format can be confusing for bash shell script programmers, because it uses
C-style variable references instead of the shell-style variable references.

Here's an example:

```bash
for (( a = 1; a < 10; a++ ))
```

Notice that there are a couple of things that don't follow the standard bash
shell `for` method:

- The assignment of the variable value can contain spaces.

- The variable in the condition isn't preceded with a dollar sign.

- The equation for the iteration process doesn't use the `expr` command format.

```bash
#!/usr/bin/bash
# testing the C-style for loop

for (( i=1; i <= 10; i++ ))
do
    echo "The next number is $i"
done
```

Output:

```bash
The next number is 1
The next number is 2
The next number is 3
The next number is 4
The next number is 5
The next number is 6
The next number is 7
The next number is 8
The next number is 9
The next number is 10
```

[[usingMultipleVariables.md]]
