---
id: parameterExpansion
aliases:
  - Parameter Expansion
tags:
  - bash
---

# Parameter Expansion

Parameter expansion is the substitution of a parameter by its value, the syntax
tells bash that you want to use the contents of the variable.

Parameters store data that can be retrieved through a symbol or a name.

`$` is the expansion character.

There are parameters and variables, Variables are actually just one kind of
parameter: parameters that are denoted by a name. Parameters that aren't
variables are called _Special Parameters_.

```bash
# Some parameters that aren't variables:
echo "My shell is $0, and has these options set: $-"
My shell is -bash, and has these options set: himB
# Some parameters that ARE variables:
echo "I am $LOGNAME, and I live at $HOME."
I am jr, and I live at /home/jr
```

`LOGNAME` is the parameter (variable) that contains your username.

`$LOGNAME` is an expression that will be replaced with the content of that
variable, which in this case is `jr`.
