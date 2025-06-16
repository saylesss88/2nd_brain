---
id: whileLoops
aliases:
  - whileLoops
tags:
  - loops
---

# whileLoops

Two other possible looping constructs are the `while...do` and `until...do`
loops.

```bash
while condition
do
  { body }
done
```

```bash
until condition
do
  { body }
done
```

- The while statement executes while the condition is true. The until statement executes
  until the condition is true—in other words, while the condition is false.

Here is a while loop that outputs 0 to 9:

```bash
N=0
while [ $N -lt 10 ] ; do
  echo -n $N
  let N=$N+1
done
```

- `-n` in the `echo` command tells `echo` not to add a newline at the end of the
  output.

- So, `echo -n $N` will print the value of `N` without adding a newline,
  allowing subsequent output to be on the same line.

Another way to output the number 0123456789 is to use an until loop as follows:

```bash
N=0
until [ $N -eq 10 ] ; do
  echo -n $N
  let N=$N+1
done
```

[[textManipulationPrograms.md]]
