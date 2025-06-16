---
id: Working With Commands
aliases:
  - Working With Commands
tags: []
---

# Working With Commands


- type—Indicate how a command name is interpreted.

- which—Display which executable program will be executed.

- man—Display a command’s manual page.

- apropos—Display a list of appropriate commands.

- info—Display a command’s info entry.

 - whatis—Display a very brief description of a command.

- alias—Create an alias for a command.

## What Exactly are Commands?

A command can be one of four things:

- **An executable program** like all those files we saw in /usr/bin. Within this
category, programs can be compiled binaries, such as programs written in
C and C++, or programs written in scripting languages, such as the shell,
Perl, Python, Ruby, and so on.

- **A command built into the shell itself**. bash supports a number of com-
mands internally called shell builtins. The cd command, for example, is a
shell builtin.

- **A shell function.** Shell functions are miniature shell scripts incorporated
into the environment. We will cover configuring the environment and
writing shell functions in later chapters, but for now just be aware that
they exist.

- **An alias**. An alias is a command that we can define ourselves, built from
other commands.

### Identifying Commands

It is often useful to know exactly which of the four kinds of commands is being
used, and Linux provides a couple of ways to find out.

- `type` command is a shell builtin that displays the kind of command the shell
  will execute, given a particular command name. It works like this:

  ```bash
  type command
  ```
- where `command` is the name of the command you want to examine.

```bash
type type
type is a shell builtin
type ls
ls is aliased to 'ls --color=tty'
type cp
cp is /usr/bin/cp
```

**which -- Display an Executable's Location**

- Sometimes more than one version of an executable is installed. The `which`
  command lets you determine the exact location of a given executable.

```bash
which ls
/usr/bin/ls
```
> [!NOTE] which works only for executable programs, not builtins or aliases that
> are substitutes for actual executable programs.

[[Getting a Command's Documentation]]
