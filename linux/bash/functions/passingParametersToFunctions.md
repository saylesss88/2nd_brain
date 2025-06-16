---
id: passingParametersToFunctions
aliases:
  - Passing parameters to functions
tags:
  - bash
  - functions
---

# Passing parameters to functions

When specifying the function in your script, you must provide the parameters on
the same command line as the function:

```bash
func1 $value1 10
```

The function can then retrieve the parameter values using the parameter
environment variables. Here's an example:

```bash
#!/usr/bin/bash
# passing parameters to functions

function addem {
  if [ $# -eq 0 ] || [ $# -gt 2 ]; then
    echo -1
  elif [ $# -eq 1 ]; then
    echo $[ $1 + $1 ]
  else
    echo $[ $1 + $2 ]
  fi
}
```
