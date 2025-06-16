---
id: LocatingCommands
aliases:
  - LocatingCommands
tags: []
---

# LocatingCommands

To find commands you type, the shell looks in your *path*.

- For commands not in your path, you can type the complete identity of location
  of the command.

  - If you know the directory that contains the command, one way to run it is to
    type the full, absolute path to the command.

For example, you run the `date` command from the `/bin` directory:

```bash
/bin/date
```

- Of course, this can be inconvenient for long path names.

  - The better way is to have commands stored in well-known directories and then
    add those directories to your shell's `PATH` environment variable.
  - The path consists of a list of directories that are checked sequentially for
    the commands you enter.

To see your current path:

```bash
echo $PATH
/home/jr/.deno/bin:/home/jr/.local/bin:/home/jr/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl
```

- Directories are separated by the `:` character.

  - Most user commands that come with Linux are stored in the `/bin`, `/usr/bin`,
  or `/usr/local/bin` directory.

  - The `/sbin` and `/usr/sbin` directories contain administrative commands
    (some Linux systems don't put those directories in regular users' `PATH`).

  - The last directory shown is the `bin` directory in the user's home
    directory.(/home/jr/bin).

> [!TIP] If you want to add your own commands or shell scripts, place them in
> the `bin` directory in your home directory (/home/jr/bin). This directory is
> automatically added to the `PATH` in some systems, although you may need to
> create that directory or add it to your `PATH` manually for some systems. So
> as long as you add the command to your `bin` with executable permission, you
> can begin using it by typing the name of the command.

- Unlike some other operating systems, Linux does not, by default, check the
  current directory for an executable before searching the path.

  - It immediately begins searching the path, and executables in the current
     directory are run only if they are in the PATH variable or you give their
     absolute or relative location.

  - The path directory order is important. Directories are checked from left
      to right.

Not all of the commands you run are located in directories in your Path, some
commands are built into the shell. Others can be overridden by creating aliases
that define any commands and options that you want the command to run.

Here is the order in which the shell checks for the commands you type:

  1. Aliases. Type `alias` to see the list of aliases. Often aliases enable you
     to define a short name for a long, complicated command.

  2. Shell reserved word. These are words reserved by the shell for special use.

  3. Function. This is a set of commands that is executed together within the
     current shell.

  4. Built-in command. Since they are built-in there is no representation of the
     command in the filesystem.
  5. Filesystem command. This command is stored in and executed from the
     computer's filesystem.(These are the commands that are indicated by the
     value of the PATH variable.)

## Finding Commands

To determine the location of a particular command, you can use the `type`
command.(if you're using a shell other than bash use the `which` command.)

For example:

```bash
type bash
/usr/bin/bash
```
- To see other locations of commands try, `which`, `case`, and `return`. If a
  command resides in several locations, you can add the `-a` option to have all
  of the known locations listed.

```bash
type -a ls
ls is an alias for eza -al --color=always --group-directories-first --icons
ls is /usr/bin/ls
```

> [!TIP] Sometimes, you run a command and receive an error message that the
> command was denied. If the command wasn't found, check spelling and your path
> variable. If permission to run the command is denied, the command may be in
> your Path variable but may not be executable.

- If a command is not in your `Path` variable, you can use `locate` to try to
  find it. Using `locate`, you can search any part of the system that is
  accessible to you.

[[CommandLineEditing.md]]
