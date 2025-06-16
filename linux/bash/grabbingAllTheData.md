---
id: grabbingAllTheData
aliases:
  - grabbingAllTheData
tags: []
---

# grabbingAllTheData

The `$*` and `$@` variables provide easy access to all your parameters. Both of
these variables include all the command line parameters within a single
variable.

- The `$*` variable takes all the parameters supplied as a single string.
  Instead of treating the parameters as multiple objects, the `$*` variable treats
  them all as one parameter.

- The `$@` variable takes all the parameters supplied as separate words in the
  same string. It allows you to iterate through the values, separating out each
  parameter supplied.

```bash
#!/bin/bash
# testing $* and $@
#
echo
count=1
#
for param in "$*"; do
  echo "\$* Parameter #$count = $param"
  count=$[ $count + 1 ]
done
#
echo
count=1
#
for param in "$@"; do
  echo "\$@ Parameter #$count = $param"
  count=$[ $count + 1 ]
done
```

test11b.sh tom jim bob
Output:

```bash
$* Parameter #1 = tom jim bob
$@ Parameter #1 = tom
$@ Parameter #2 = jim
$@ Parameter #3 = bob
```

[[beingShifty.md]]
