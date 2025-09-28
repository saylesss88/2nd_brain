---
title: Setting the PATH Environment Variable
date: 2024-10-29
tags: [tag1, tag2]
---

# Setting the PATH Environment Variable

All the directories that your system searches for programs and commands are the
`$PATH` each directory is separated by a colon:

```bash
echo $PATH
/home/jr/go/bin:/home/jr/.local/bin:/home/jr/go/bin:/home/jr/.local/bin:/home/jr/.bun/bin:/home/jr/.pyenv/shims:/home/jr/go/bin:/home/jr/.local/bin:/home/jr/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:/home/jr/.local/share/flatpak/exports/bin:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/jr/.local/share/bin
```

You can append new search directories to the existing PATH, all you have to do is
reference the original `PATH`:

```bash
PATH=$PATH:/home/jr/Scripts

echo $PATH
/home/jr/go/bin:/home/jr/.local/bin:/home/jr/go/bin:/home/jr/.local/bin:/home/jr/.bun/bin:/home/jr/.pyenv/shims:/home/jr/go/bin:/home/jr/.local/bin:/home/jr/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:/home/jr/.local/share/flatpak/exports/bin:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/jr/.local/share/bin:/home/jr/Scripts
```

> [!TIP] If you want your program's location to be available to subshells, be
> sure to export your modified `PATH` environment variable.

- A common trick for programmers is to include the single dot symbol in their
  `PATH`. The single dot represents the current directory:

```bash
PATH=$PATH:.
```

### Make PATH changes persistent

Add the logic to your `.bashrc` or `.zshrc`

When you start a bash shell, it checks several files for commands. These files
are called _startup files_ or _environment files_.

- The startup files that bash processes depend on the method you use to start
  the bash shell. You can start a bash shell in three ways:

- As a default login shell at login time

- As an interactive shell that is started by spawning a subshell

- As a non-interactive shell to run a script

[[theLoginShellProcess.md]]
