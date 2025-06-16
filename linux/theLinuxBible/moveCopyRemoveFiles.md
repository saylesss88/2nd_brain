---
id: moveCopyRemoveFiles
aliases:
  - Moving, Copying, and Removing Files
tags: []
---

# Moving, Copying, and Removing Files

- To change the location of a file, use the `mv` command.

- To copy a file from one location to another, use the `cp` command.

- To remove a file, use the `rm` command.

Examples:

```bash
mv abc def
mv abc ~  # moves abc to your home directory
mv /home/jr/mymemos /home/jr/Documents/
```

- The first `mv` command moves the file `abc` to the file `def` in the same
  directory (renaming it)
- The last `mv` command moves the file `mymemos` to the directory
  `/home/jr/Documents/`

By default, the `mv` command overwrites any existing files if the file to which
you are moving exists. However, many distros alias the `mv` command to use the
`-i` option (which causes `mv` to prompt you before overwriting a file).

Check with `alias mv`
`mv -iv` : This iv interactive and verbose.



