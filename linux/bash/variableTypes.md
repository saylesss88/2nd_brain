---
id: variableTypes
aliases:
  - Variable Types
tags:
  - bash
  - variables
---

# Variable Types

- Array: `declara -a` variable: The variable is an array of strings.

- Associative array: `declare -A` variable: The variable is an associative
  array of strings.

- Integer: `declare -i` variable: The variable is an integer. Assigning values
  to this variable automatically triggers Arithmetic Evaluation.

- Read Only: `declare -r` variable: The variable can no longer be modified or
  unset.

- Export: `declare -x` variable: The variable is marked for export which means
  it will be inherited by any child process.

```bash
$ a=5; a+=2; echo "$a"; unset a
52
$ a=5; let a+=2; echo "$a"; unset a
7
$ declare -i a=5; a+=2; echo "$a"; unset a
7
$ a=5+2; echo "$a"; unset a
5+2
$ declare -i a=5+2; echo "$a"; unset a
7
```

The use of `declare -i` and `declare -a` are rarely used anymore.

- For `declare -i` `((...))` or `let` is used.

- For `declare -r`, it is sufficient to write `array=(...)` and Bash will know
  that the variable is now an array.

- The exception is the associative array, which must be declared explicitly:
  `declare -A myarray`

[[parameterExpansion.md]]
