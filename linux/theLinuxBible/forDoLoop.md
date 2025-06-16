---
id: forDoLoop
aliases:
  - forDoLoop
tags:
  - loops
---

# forDoLoop

Loops are used to perform actions over and over again until a condition is met or until all
data has been processed. One of the most commonly used loops is the for...do loop. It
iterates through a list of values, executing the body of the loop for each element in the list.
The syntax and a few examples are presented here:

```bash
for VAR in LIST
do
    { body }
done
```

The `for` loop assigns the values in the LIST to VAR one at a time. Then, for
each value, the body in braces between do and done is executed. VAR can be any
variable name, and LIST can be composed of pretty much any list of values or
anything that generates a list.

```bash
for NUMBER in 0 1 2 3 4 5 6 7 8 9
do
  echo The number is $NUMBER
done

for FILE in `/bin/ls`
do
  echo $FILE
done
```

You can also writ it this way, which is somewhat cleaner:

```bash
for NAME in John Paul Ringo George ; do
  echo $NAME is my favorite Beatle
done
```

- Each element in the LIST is separated by a space.

- [!] This can cause trouble
  if you’re not careful because some commands, such as ls -l, output multiple fields per
  line, each separated by white space. The string done ends the for statement

[[whileLoops.md]]
