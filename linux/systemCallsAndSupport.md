---
id: System Calls and Support
aliases:
  - System Calls and Support
tags: []
---

# System Calls and Support

There are several other kinds of kernel features available to user processes.
For example, system calls (or syscalls) perform specific tasks that a user process
alone cannot do well or at all. For example, the acts of opening, reading, and
writing files all involve system calls.

Two system calls, fork() and exec(), are important to understanding how
processes start:

  - `fork()` When a process calles `fork()`, the kernel creates a nearly
    identical copy of the process.

  - `exec()` When a process calles `exec(program)`, the kernel loads and starts
    `program`, replacing the current process.

Other than `init`, all new user processes on a Linux system start as a result of
`fork()`, and most of the time, you also run `exec()` to start a new program
instead of running a copy of an existing process.

  - A simple example is any program that you run at the command line, such as
    the `ls` command to show the contents of a directory.

  - When you enter `ls` into a terminal window, the shell that's running inside
    the terminal window calls `fork()` to create a copy of the shell, and then
    the new copy of the shell calles `exec(ls)` to run `ls`.

> [!NOTE] System calls are normally denoted in parentheses. The process asking
> the kernel to create another process must perform a `fork()` system call. This
> notation derives from the way the call would be written in the C language.
> Just remember that a system call is an interaction between a process and the
> kernel.

The kernel also supports user processes with features other than
traditional system calls, the most common of which are pseudodevices.
Pseudodevices look like devices to user processes, but they’re imple-
mented purely in software. This means they don't need to be in the kernel, but
they are usually there for practical reasons. For example, the kernel random
number generator device (/dev/random) would be difficult to implement securely
with a user process.

>[!Note] Technically, a user process that accesses a pseudodevice must use a
>system call to open the device, so the processes can't entirely avoid system
>calls.

[[User Space.md]]

