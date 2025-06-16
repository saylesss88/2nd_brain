---
id: Executing Commands
aliases:
  - Executing Commands
tags: []
---

# Executing Commands

Bash determines the type of program that is to be executed. Normal programs are
system commands that exist in compiled form on your system.

When such a program is executed, a new process is created because Bash makes
an exact copy of itself. This child process has the same environment as its
parent, only the process ID is different. This procedure is called _forking_

After the forking process, the address space of the child process is overwritten
with the new process data. This is done through an exec call to the system.
The fork-and-exec mechanism thus switches an old command with a new, while the
environment in which the new program is executed remains the same, including
configuration of input and output devices, environment variables and priority.
This mechanism is used to create all UNIX processes, so it also applies to the Linux
operating system. Even the first process, init, with process ID 1, is forked
during the boot procedure in the so-called bootstrapping procedure.

[[shellBuiltins.md]]
