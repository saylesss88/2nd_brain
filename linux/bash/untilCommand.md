---
id: untilCommand
aliases:
  - Until Command
tags:
  - bash
  - commands
---

# Until Command

Syntax:

```bash
until test commands
do
    other commands
done
```

- You can have more than one test command in the `until` command statement. Only
  the exit status of the last command determines if the bash shell executes the
  other commands defined.

```bash
#!/usr/bin/bash
# until command

var1=100

until [ $var1 -eq 0 ]
do
    echo $var1
    var1=$[ $var1 - 25 ]
done
```

100
75
50
25

```bash
#!/usr/bin/bash
# until command

var1=100

until echo $var1
    [ $var1 -eq 0 ]
do
    echo Inside the loop: $var1
    var1=$[ $var1 - 25 ]
done
```

100
Inside the loop: 100
75
Inside the loop: 75
50
Inside the loop: 50
25
Inside the loop: 25
0

[[nestingLoops.md]]
