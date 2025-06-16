---
id: creatingAFunction
aliases:
  - Creating a function
tags:
  - bash
  - functions
---

# Creating a function

There are two formats you can use to create functions in Bash. The first uses
the keyword `function`, along with the function name you assign to the block of
code:

```bash
function name {
    commands
}
```

The second format more closely follows how functions are defined in other
programming languages:

```bash
name() {
commands
}
```

- The empty parentheses after the function name indicate that you're defining a
  function.

## Using functions

```bash
#!/usr/bin/bash

function func1 {
  echo "This is an example of a function."
}

count=1
while [ $count -le 5 ]; do
  func1
  count=$(( $count + 1 ))
done

echo "This is the end of the loop."
func1
echo "Now this is the end of the script."
```

The function definition doesn't have to be the first thing in your script, but
be careful. If you attempt to use a function before it's defined, you'll get an
error message:

```bash
#!/usr/bin/bash

count=1
echo "This line comes before the function definition."

function func1 {
  echo "This is an example of a function."
}

while [ $count -le 5 ]; do
  func1
  count=$(( $count + 1 ))
done

echo "This is the end of the loop."
func2
echo "Now this is the end of the script."

function func2 {
  echo "This is an example of a function"
}
```

```bash
Output:
./test2
This line comes before the function definition.
This is an example of a function.
This is an example of a function
This is an example of a function
This is an example of a function
This is an example of a function
This is the end of the loop.
bash: func2: command not found
Now this is the end of the script.
```
