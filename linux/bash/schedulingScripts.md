---
id: schedulingScripts
aliases:
  - Scheduling Scripts
tags: []
---

# Scheduling Scripts

The Linux system provides a couple of ways to run a script at a preselected
time: the `at` command and the `cron` table.

The `at` command allows you to run a script at a specific time.

- The `atd` daemon checks a special directory on the system (usually
  /var/spool/at) every minute for jobs submitted using the `at` command.

`at` command syntax:

```bash
at [-f filename] time
```

- By default, the `at` command submits input from `STDIN` to the queue. You can
  specify a filename used to read commands (your script file) using the -f
  parameter.

- The `time` parameter specifies when the script should be executed.
