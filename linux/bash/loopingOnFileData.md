---
id: loopingOnFileData
aliases:
  - Looping on file data
tags:
  - bash
  - loops
---

# Looping on file data

Often you must iterate through items stored inside a file. This requires
combining:

- Nested loops

- Changing the IFS environment variable.

By changing the `IFS` variable, you can force the `for` command to handle each
line in the file as a separate item for processing, even if the data contains
spaces.

The classic example of this is processing data in the /etc/passwd ﬁle. This requires that
you iterate through the /etc/passwd ﬁle line by line and then change the IFS variable
value to a colon so you can separate the individual components in each line.

The following example shows how to do this:

```bash
#!/usr/bin/bash
# changing the IFS value

IFS.OLD=$IFS
IFS=$'\n'

for entry in $(cat /etc/passwd)
do
  echo "Values in $entry -"
  IFS=:
  for value in $entry
  do
    echo "  $value"
  done
done
```

- This script uses two different `IFS` values to parse the data. The first IFS
  value parses the individual lines in the `/etc/passwd` file. The inner `for`
  loop next changes the `IFS` value to a colon, which allows you to parse the
  individual values in each line.

[[controllingTheLoop.md]]
