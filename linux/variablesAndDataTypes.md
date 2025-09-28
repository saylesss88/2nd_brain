---
id: Variables and data types in Bash
aliases:
  - Variables and data types in Bash
tags: []
---

# Variables and data types in Bash

Variables let you store data. You can use variables to read, access, and
manipulate data throughout your script.

There are no data types in Bash. In Bash, a variable is capable of storing
numeric values, individual characters, or strings of characters.

In Bash, you can use and set variable values in the following ways:

1. Assign the value directly:

```bash
country=Netherlands
```

2. Assign the value based on the output obtained from a program or command,
using command substitution. Note that $ is required to access an existing
variable's value.

```bash
same_country=$country
```

This assigns the value of `country` to the new variable `same_country`.

To access the variable value, append `$` to the variable name:

```bash
country=Netherlands
echo $country
# output
Netherlands
new_country=$country
echo $new_country
# output
Netherlands
```
