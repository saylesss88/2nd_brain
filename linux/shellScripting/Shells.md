---
id: Shells
aliases:
  - Shells
tags: []
---

# Shells

The file `/etc/shells` gives an overview of known shells on a Linux system.

```bash
cat /etc/shells
```

Your default shell is set in the `/etc/passwd` file, like this line for user
mia:

`mia:L2NOfqdlPrHwE:504:504:Mia Maya:/home/mia:/bin/bash`

  - To switch from one shell to another, just enter the name of the new shell in
    the active terminal.

  - The system finds the directory where the name occurs using the `PATH` settings,
    and since a shell is an executable, it can be executed.

[[interactiveShells.md]]


