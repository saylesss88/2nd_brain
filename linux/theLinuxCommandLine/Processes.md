---
title: Processes
date: 2024-10-12
tags: [tag1, tag2]
---

# Processes

Processes are how Linux organizes the different programs waiting for their turn
at the CPU.

When the system starts, the kernel initiates a process called the init. init,
in turn, runs a series of shell scripts (located in `/etc`) called _init
scripts_, which start system services.

- Many of the services are implemented as _daemon programs_, programs that run
  in the background without an interface.

- The fact that a program can launch other programs is expressed in the process
  scheme as _parent process_ producing a _child process_.

## Viewing processes with ps

TTY is short for _teletype_ and refers to the _controlling terminal_ for the
process.

- The TIME field is the amount of CPU time consumed by the process.

```bash
ps x
```

- The `x` option tells `ps` to show all of our processes regardless of what
  terminal (if any) they are controlled by.

- The `?` in the TTY column indicates no controlling terminal.
