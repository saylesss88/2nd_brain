---
title: Understanding the interactive shell process
date: 2024-10-29
tags: [tag1, tag2]
---

# Understanding the interactive shell process

If you start a bash shell without logging into a system (if you just type bash
at a CLI prompt, for example), you start what's called an _interactive shell_.

- If bash is started as an interactive shell, it doesn't process the
  `/etc/profile` file. Instead it only checks for the `.bashrc` file in the user's
  HOME directory.

- The `.bashrc` file does two things. First, it checks for a common `bashrc`
  file in the `/etc` directory. Second, it provides a place for the user to enter
  personal command aliases and private script functions.

## Understanding the non-interactive shell process

This is the shell where the system can start to execute a shell script.

- non-interactive is different in that there isn't a CLI prompt to worry about.
  However, you may want to run specific startup commands each time you start a
  script on your system.

> [!TIP] Scripts can be executed in different ways. Only some execution methods
> start a subshell.

- When the shell starts a non-interactive subshell process, it checks the
  `BASH_ENV` environment variable. If one is present, the shell executes the
  file's commands, which typically include variables set for the shell scripts.

- If the `BASH_ENV` variable isn't set, remember that some execution methods
  start a subshell, also called a child shell. A child shell inherits its parent
  shell's exported variables.
