---
id: trappingSignals
aliases:
  - Trapping signals
tags: []
---

# Trapping signals

Instead of allowing your script to leave signals ungoverned, you can trap then
when they appear and perform other commands.

- The `trap` command allows you to specify which Linux signals your shell script
  can watch for and intercept from the shell. If the script receives a signal
  listed in the `trap` command, it prevents it from being processed by the shell
  and instead handles it locally.

The format of the `trap` command is:

```bash
trap commands signals
```

- On the `trap` command-line, you just list the commands you want the shell to
  execute, along with a space-separated list of signals you want to trap.

```bash
#!/usr/bin/bash
# Trapping the script exit
#
trap "echo 'Goodbye!'" EXIT
#
count=1
while [ $count -le 5 ]; do
  echo "Loop #$count"
  sleep 1
  count=$[ $count + 1 ]
done
```

```bash
# Output:
./test2.sh
Loop #1
Loop #2
Loop #3
Loop #4
Loop #5
Goodbye!
```

- When the script gets to the normal exit point, the trap is triggered, and the
  shell executes the command you specify on the `trap` command-line. The `EXIT`
  trap also works if you prematurely exit the script.

## Modify or remove a trap

```bash
#!/usr/bin/bash
# Modifying a trap
#
trap "echo ' Sorry... Ctrl-C is trapped." SIGINT
#
count=1
while [ $count -le 5 ]; do
  echo "Loop #$count"
  sleep 1
  count=$[ $count + 1 ]
done
#
trap "echo ' I modified the trap!'" SIGINT
#
count=1
while [ $count -le 5 ]; do
  echo "Second Loop #$count"
  sleep 1
  count=$[ $count + 1 ]
done
#
```

[[runningScriptsInBackgroundMode.md]]
