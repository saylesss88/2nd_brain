---
id: listingFiles&Directories
aliases:
  - listingFiles&Directories
tags: []
---

# listingFiles&Directories

- There are many options available with the `ls` command that allow you to
  gather different sets of files and directories as well as to view different
  kinds of information about them.

  - By default the `ls` command output shows you all non-hidden files and
    directories contained in the cwd.

  - When you type `ls`, however, many Linux systems assign an alias ls to add
    options. To see if `ls` is an alias, type `alias ls`.

```bash
alias ls
alias ls='ls --color=auto'
```

- The `--color=auto` option causes different types of files and directories to
  be displayed in different colors.

Return to the `$HOME/test` dir and add:

```bash
cd $HOME/test
touch scriptx.sh apple
chmod 755 scriptx.sh
mkdir Stuff
ln -s apple pointer_to_apple
ls
drwxr-xr-x - jr 30 Aug 09:42  Stuff
.rw-r--r-- 0 jr 30 Aug 09:42  apple
.rw-r--r-- 0 jr 29 Aug 12:29  banana
.rw-r--r-- 0 jr 29 Aug 12:29  grape
.rw-r--r-- 0 jr 29 Aug 12:29  grapefruit
lrwxrwxrwx - jr 30 Aug 09:42  pointer_to_apple -> apple
.rwxr-xr-x 0 jr 30 Aug 09:42  scriptx.sh
.rw-r--r-- 0 jr 29 Aug 12:29  watermelon

```

- The `Stuff` directory shows up in blue, `pointer_to_apple` (a symbolic link)
  appears in aqua, and the `scriptx.sh` file appears in green.

- All other regular files show up in black.
