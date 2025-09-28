---
id: theCommandPath
aliases: []
tags: []
---

- [!] The Command Path

`PATH` is a special environment variable that contains the _command path_, a
list of system directories that the shell searches when trying to locate a
command.

- For example, when you run the `ls` command, the shell searches the directories
  listed in `PATH` for the `ls` program. If programs with the same name appear
  in several directories in the path, the shell will use the first one.

## Adding directories to `PATH`

```bash
PATH=dir:$PATH
```

- Add `dir` to the beginning of the path so the shell looks in `dir` before
  looking in anyh other PATH directories.

```bash
PATH=$PATH:dir
```

- Or you can append a directory name to the end of the `PATH` variable, causing
  the shell to look in `dir` last.

> [!WARNING] You can accidentally wipe out your entire path if you mistype $PATH when modifying
> your path. If this happens, don’t panic! The damage isn’t permanent; you can just
> start a new shell. (For a lasting effect, you need to mistype it when editing a certain
> configuration file, and even then it isn’t difficult to rectify.) The easiest way to return
> to normal is to close the current terminal window and start another.
