---
id: jobControl
aliases:
  - Job Control
tags: []
---

# Job Control

After you stop a job, the Linux system lets you either kill or restart it.

- You can kill a process by using the `kill` command. Restarting a stopped
  process requires that you send it a `SIGCONT` signal.

- The function of starting, stopping, killing, and resuming jobs is called _job
  control_.
