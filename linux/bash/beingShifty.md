---
id: beingShifty
aliases:
  - Being Shifty
tags: []
---

# Being Shifty

The bash shell provides the `shift` command to help you manipulate command line
parameters.

- When you use the `shift` command, it moves each parameter variable one
  position to the left by default. So, the variable `$3` will be moved to `$2` and
  `$2` will be moved to `$1`. And the value for the variable `$1` is discarded
  (note that the value for the variable $0, the program name, remains unchanged).

```bash
#!/usr/bin/bash
# shift command
echo
count=1
while [ -n "$1" ]; do
  echo "Parameter $count: $1"
  count=$(( count + 1 ))
  shift
done
```
