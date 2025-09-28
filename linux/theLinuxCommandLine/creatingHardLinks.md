---
id: Creating Hard Links
aliases:
  - Creating Hard Links
tags: []
---

# Creating Hard Links

```bash
ln fun fun-hard
ln fun dir1/fun-hard
ln fun dir2/fun-hard
```

```bash
ls -l
drwxr-xr-x    - jr 26 Aug 15:13  dir1
drwxr-xr-x    - jr 26 Aug 15:13  dir2
.rw-r--r-- 2.1k jr 26 Aug 15:00  fun
.rw-r--r-- 2.1k jr 26 Aug 15:00  fun-hard
```

This effectively places fun in both directories.

When thinking about hard links, it is helpful to imagine that files are made up
of two parts: the data part containing the file's contents and the name part,
which holds the file's name.

    When we create hard links, we are actually creating additional name parts
    that all refer to the same data part.

    The system assigns a chain of disk blocks to what is called an inode, which
    is then associated with the name part.

    Each link therefor refers to a specific inode containing the files    contents.

The `ls` command has a way to reveal this information. It is invoked with the
`-i` option.

```bash
ls -li
1544408 drwxr-xr-x    - jr 26 Aug 15:13  dir1
1544409 drwxr-xr-x    - jr 26 Aug 15:13  dir2
1544411 .rw-r--r-- 2.1k jr 26 Aug 15:00  fun
1544411 .rw-r--r-- 2.1k jr 26 Aug 15:00  fun-hard
```

The first fielld is the inode number, and as you can see, both fun and fun-hard
share the same inode number, which confirms they are the same file.

[[Creating Symbolic Links.md]]
