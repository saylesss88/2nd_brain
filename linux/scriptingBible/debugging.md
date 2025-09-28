---
id: debugging
aliases:
  - Debugging
tags:
  - bash
  - debugging
---

# Debugging

The most common technique is to start up a subshell with the `-x` option, which
will run the entire script in debug mode. Traces of each command plus its
arguments are printed to standard output after the commands have been expanded
but before they are executed.

```bash
#!/bin/bash commented-script1.sh
# This script clears the terminal, displays a greeting and gives information
# about currently connected users. The two example variables are set and displayed.

clear # clear terminal window

echo "The script starts now."

echo "Hi, $USER!" # dollar sign is used to get content of variable
echo

echo "I will now fetch you a list of connected users:"
echo
w # show who is logged on and
echo # what they are doing

echo "I'm setting two variables now."
COLOUR="black" # set a local shell variable
VALUE="9" # set a local shell variable
echo "This is a string: $COLOUR" # display content of variable
echo "And this is a number: $VALUE" # display content of variable
echo

echo "I'm giving you back your prompt now."
echo
```

## Debugging on the whole script

```bash
bash -x commented-script1.sh
```

## Debugging on part(s) of a script

Using the `set` built-in you can run in normal mode those portions of the script
of which you are sure they are without fault, and display debugging information
only for the troublesome zones.

Say we are unsure what the `w` command will do, we could do the following:

```bash
set -x
w
set +x
```
