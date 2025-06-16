---
title: .bash_profile
date: 2024-10-29
tags: [tag1, tag2]
---

# .bash_profile

```bash
$ cat $HOME/.bash_profile
# .bash_profile

# Get the aliases and functions
if [ -f ~/.bashrc ]; then
. ~/.bashrc
fi

# User specific environment and startup programs
PATH=$PATH:$HOME/bin
export PATH
```

- The `.bash_profile` startup file first checks to see if the startup file
  `.bashrc` is present in the `HOME` directory. If it's there, the startup file
  executes the commands in it.

[[understandingTheInteractiveShell.md]]
