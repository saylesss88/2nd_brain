---
id: handlingVariablesInAFunction
aliases:
  - Handling variables in a function
tags:
  - bash
  - functions
---

# Handling variables in a function

The `scope` of a variable can cause problems for shell script programmers.

- The scope is where the variable is visible.

- Variables defined in functions can have a different scope than regular
  variables. They can be hidden from the rest of the script.

Functions use two types of variables:

- Global

- Local

## Global Variables

If you define a global variable in the main section of a script, you can
retrieve its value inside a function. Likewise, if you define a global variable
inside a function, you can retrieve its value in the main section of the script.

By default, any variables you define in the script are global variables.
Variables defined outside of a function can be accessed inside a function:

```bash
#!/usr/bin/bash
# using a global variable to pass a value

function dbl {
  value=$[ $value * 2 ]
}

read -p "Enter a value: " value
dbl
echo "The value is: $value"
```

- The `$value` variable is defined outside of the function and assigned a value
  outside of the function.

- When the `dbl` function is called, the value of the `$value` variable is still
  valid inside the function. When the variable is assigned a new value inside
  the function, that new value is still valid when the script references the
  variable.

  - This can be dangerous, especially if you intend to use your functions in
    different scripts.
