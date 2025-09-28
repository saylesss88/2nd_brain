---
id: usingMultipleVariables
aliases:
  - Using Multiple Variables
tags: []
---

# Using Multiple Variables

The C-style `for` command also allows you to use multiple variables for
iteration. The loop handles each variable separately, allowing you to define a
different iteration process for each variable. Although you can have multiple
variables, you can define only one condition in the for loop:

```bash
#!/usr/bin/bash
# multiple variables

for (( a=1, b=10; a <= 10; a++, b-- ))
do
    echo "$a - $b"
done
```

Output:

```bash
1 - 10
2 - 9
3 - 8
4 - 7
5 - 6
6 - 5
7 - 4
8 - 3
9 - 2
10 - 1
```

[[whileCommand.md]]
