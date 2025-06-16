---
id: case_command
aliases:
  - `case` command
tags: []
---

# `case` command

Often, you'll need to evaluate a variable's value, looking for a specific value
within a set of possible values. In this scenario, you end up having to write a
lengthy `if-then-else` statement like this:

```bash
#!/bin/bash
# looking for a possible value
#
if [ $USER = "rich" ]
then
  echo "Welcome $USER"
  echo "Please enjoy your visit"
elif [ $USER = "barbara" ]
then
  echo "Welcome $USER"
  echo "Please enjoy your visit"
elif [ $USER = "testing" ]
then
  echo "Special testing account"
elif [ $USER = "jessica" ]
then
  echo "Do not forget to log off when you're done"
else
  echo "Sorry, you are not allowed here"
fi
```

## Syntax

```bash
case variable in
pattern1 | pattern2) commands1;;
pattern3) commands2;;
*) default commands;;
esac
```

- The `case` command compares the variable specified against the different
  patterns. If the variable matches the pattern, the `case` command executes the
  commands specified for the pattern.

```bash
#!/bin/bash
# using the case command
#
case $USER in
rich | barbara)
  echo "Welcome, $USER"
  echo "Please enjoy your visit";;
testing)
  echo "Special testing account";;
jessica)
  echo "Do not forget to log off when you're done";;
*)
  echo "Sorry, you are not allowed here";;
esac
```
