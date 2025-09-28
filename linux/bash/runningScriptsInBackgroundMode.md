---
id: runningScriptsInBackgroundMode
aliases:
  - Running scripts in the background mode
tags: []
---

# Running scripts in the background mode

```bash
#!/usr/bin/env bash
# run script in the background
#
count=1
while [ $count -le 10 ]; do
  sleep 1
  count=$(( $count + 1 ))
done
```

```bash
# Output:
./test4.sh &
[1] 1234
```

- When you place an & after the command, it separates the command from the shell
  and runs it as a separate background process on the system.

- The line `[1] 1234` is the PID of the background process.

- When the background process is finished, it displays a message to the
  terminal:

```bash
[1]+  Done                    ./test4.sh
```

- This shows the job number and the status of the job (Done), along with the
  command used to start the job.
