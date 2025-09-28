---
id: getInfoAboutCommands
aliases:
  - Get Info About Commands
tags:
  - commands
---

# Get Info About Commands

How do you know which commands are available, which options they use, or how to
use advanced features? Here are some places that you can look:

- **Check the PATH** : Type `echo $PATH` to see all of the directories
  containing commands that are immediately available. Listing the contents of
  those directories displays most standard Linux commands. For example:

```bash
ls /bin
arch dd fusermount loadkeys mv ...
```

- **Use the help command** : Some commands are built into the shell, so they
  don't appear in a directory. The `help` command lists those commands and shows
  options available with each of them. (enter `help | less` to page through the
  list.)
