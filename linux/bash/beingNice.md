---
id: beingNice
aliases:
  - Being Nice
tags: []
---

# Being Nice

In a multitasking operating system (which Linux is), the kernel is responsible
for assigning CPU time for each process running on the system.

- The _scheduling priority_ is the amount of CPU time the kernel assigns to the
  process relative to the other processes. `-20` is the highest priority. and
  `+19` is the lowest priority.
  - By default, the bash shell starts all processes with a scheduling priority
    of 0.

## Using the nice command

The `nice` command allows you to set the scheduling priority of a command as you
start it. To make a command run with less priority, just use the `-n` command
line option for `nice` to specify a new priority level:

```bash
$ nice -n 10 ./test4.sh > test4.out &
[1] 4973
$
$ ps -p 4973 -o pid,ppid,ni,cmd

PID PPID   NI   CMD
4973 4721  10   /usr/bin/bash ./test4.sh
```

- The output of the `ps` command shows that the process has a scheduling
  priority of `10`.

- Normal users can only increase the nice level (decrease the priority).

## Using the renice command

The `renice` command allows you to change the scheduling priority of a running
process.

- You can only renice processes that you own.

- You can only renice your processes to a lower priority.

- The root user can renice any process to any priority.

[[schedulingScripts.md]]
