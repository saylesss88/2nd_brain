---
id: searchingForFilesWithFind
aliases: []
tags:
  - commands
---

The `find` command is the best one for searching your filesystem for files based
on a variety of attributes.

- Run on a line by itself, the find command finds all files and directories below
  the current directory.

  - If you want to search from a particular point in the directory tree, just add
    the name of the directory you want to search (such as `/etc`)

  - After the files are found, you can act on those files as well (using -exec
    or -okay option) by running any commands you want on them.

## Finding files by name

To find files by name, you can use the `-name` and `-iname` options. The `-name`
options matches exactly.

```bash
# find /etc -name passwd
/etc/pam.d/passwd
/etc/passwd
# find /etc -iname '*passwd*'
/etc/pam.d/passwd
/etc/passwd-
/etc/passwd.OLD
/etc/passwd
/etc/MYPASSWD
/etc/security/opasswd
```

- The `-iname` option matches any combination of upper- and lowercase. Using
  asterisks, you can match any filename that includes the word `passwd`.

[[findingFilesByTime.md]]
