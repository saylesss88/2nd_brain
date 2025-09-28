---
id: command_options
aliases:
  - Command Options
tags: []
---

# Command Options

As you extract each individual parameter, use the `case` statement to determine
when a parameter is formatted as an option:

```bash
#!/usr/bin/bash
# extracting command line options as parameters
#
echo
while [ -n "$1" ]; do
  case "$1" in
    -a) echo "Found the -a option";;
    -b) echo "Found the -b option";;
    -c) echo "Found the -c option";;
     *) echo "$1 is not an option";;
  esac
  shift
done
```

./test15a.sh -a -b -c -d
Found the -a option
Found the -b option
Found the -c option
d is not an option

- The `case` statement checks each parameter for valid options. When one is
  found, the appropriate commands are run in the `case` statement.
