---
id: read_command
aliases:
  - read_command
tags: []
---

# read_command

The `read` command accepts input either from standard input (keyboard) or from a
another file descriptor. After receiving input from the keyboard, the `read`
command places the data into a variable.

```bash
#!/usr/bin/bash
# testing the read command
#
echo -n "Enter your name: "
read name
echo "Hello $name, welcome to my program."
#
```

- The `-n` option suppresses the newline character at the end of the string,
  allowing the script user to enter data immediately after the string, instead of
  on the next line. This gives your scripts a more form-like appearance.

The `read -p` option allows you to specify a prompt directly in the `read`
command line:

```bash
#!/usr/bin/bash

read -p "Please enter your age: " age
days=$(( age * 365 ))
echo "That makes you over $days days old!"
```
