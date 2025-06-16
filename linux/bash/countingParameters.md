---
id: countingParameters
aliases:
  - Counting Parameters
tags:
  - bash
---

# Counting Parameters

Instead of testing each parameter, you can count how many parameters were
entered on the command line.

- The special `$#` variable can be used to count the number of command line
  parameters included when the script was run.

```bash
#!/usr/bin/bash
# getting the number of parameters
echo There were $# parameters supplied
test8.sh 1 2 3 4 5
```

Output:

There were 5 parameters supplied

Now we can test the number of parameters present before trying to use them:

```bash
#!/usr/bin/bash
# Testing parameters
if [ $# -ne 2 ]; then
  echo
  echo Usage: test9.sh a b
  echo
else
  total=$(( $1 + $2 ))
  echo
  echo The total is $total
  echo
fi
```

To grab the last parameter, use `${!#}`:

```bash
#!/usr/bin/bash
# getting the last parameter
params=$#
echo
echo The last parameter is $params
echo The last parameter is ${!#}
echo
```

[[grabbingAllTheData.md]]
