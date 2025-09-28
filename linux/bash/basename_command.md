---
id: basename_command
aliases:
  - basename command
tags:
  - bash
  - commands
---

# basename command

The `basename` command returns just the script's name without the path:

```bash
#!/usr/bin/bash
# Using basename with the $0 parameter
#
name=$(basename $0)
echo "The script's name is: $name"
```

**Output:**
bash /home/jr/test5b.sh
The script's name is: test5b.sh

- When the script assumes there is data in a parameter variable, and no data is
  present, you'll most likely get an error. This is a poor way to write scripts,
  always check your parameters to make sure the data is there before using it:

```bash
#!/usr/bin/bash
# testing parameters before use
#
if [ -n "$1" ]; then
  echo Hello $1, glad to meet you.
else
  echo "Sorry, you didn't identify yourself."
fi
```

- The `-n` test evaluation was used to check for data in the `$1` parameter.
