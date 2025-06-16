---
id: exitingTheScript
aliases:
  - Exiting the Script
tags:
  - bash
  - scripting
---

# Exiting the Script

Every command that runs in the shell uses an `exit status` to indicate to the
shell that it's finished processing.

- The exit status is an integer value between 0 and 255 that's passed by the
  command to the shell when the command finishes running. You can capture this
  value in your scripts.

## Checking the exit status

Linux provides the `$?` special variable that holds the exit status value from
the last command that executed.

- You must view or use the `$?` variable immediately after the command you want
  to check. It changes values to the exit status of the last command executed by
  the shell:

```bash
date
Sat Jan 15 10:01:30 EDT 2014
echo $?
0
```

- By convention, the exit status of a command that successfully completes is
  zero.

- If a command completes with an error, then a positive integer value is placed
  in the exit status:

```bash
asdfg
-bash: asdfg: command not found
echo $?
127
```

- Invalid command returns an exit status of 127.

- 126 indicates that the user didn't have the proper permissions set to execute
  the command.

The `exit` command allows you to specify an exit status when your script ends:

```bash
#!/bin/bash
# testing the exit status
var1=10
var2=30
var3=$[$var1 + $var2]
echo The answer is $var3
exit 5
```
