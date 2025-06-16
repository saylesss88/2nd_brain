---
id: Creating Symbolic Links
aliases:
  - Creating Symbolic Links
tags: []
---

# Creating Symbolic Links

Symbolic links were created to overcome the two disadvantages of hard
links: Hard links cannot span physical devices, and hard links cannot refer-
ence directories, only files. Symbolic links are a special type of file that con-
tains a text pointer to the target file or directory.
Creating symbolic links is similar to creating hard links:

```bash
ln -s fun fun-sym
ln -s ../fun dir1/fun-sym
ln -s ../fun dir2/fun-sym
```

> [!NOTE], when we create a symbolic link, we are creating a text
> discription of where the target file is relative to the symbolic link.

It's easier to see if we look at the `ls` output:

```bash
ls -l dir1
.rw-r--r-- 2.1k jr 26 Aug 15:00  fun-hard
lrwxrwxrwx    - jr 26 Aug 15:32  fun-sym -> ../fun
```

- The listing for fun-sym in dir1 shows that it is a symbolic link by the
  leading l in the first field and the fact that it points to ../fun, which is
  correct.

    - Relative to the location of fun-sym, fun is in the directory above it.
      orthe relative pathnames.  Using relative pathnames is more desirable
      because it allows a directory containing symbolic links to be renamed
      and/or moved without breaking the links.

Symlincs can also refer to directories:

```bash
ln -s dir1 dir1-sym
ls -l
╭─jr@jr in ~/playground took 10ms
 ╰─λ ls
drwxr-xr-x    - jr 26 Aug 15:32  dir1
lrwxrwxrwx    - jr 26 Aug 17:50  dir1-sym -> dir1
drwxr-xr-x    - jr 26 Aug 15:33  dir2
.rw-r--r-- 2.1k jr 26 Aug 15:00  fun
.rw-r--r-- 2.1k jr 26 Aug 15:00  fun-hard
lrwxrwxrwx    - jr 26 Aug 15:32  fun-sym -> fun
```

## Removing files and directories

```bash
rm fun-hard
ls -l
drwxr-xr-x    - jr 26 Aug 15:32  dir1
lrwxrwxrwx    - jr 26 Aug 17:50  dir1-sym -> dir1
drwxr-xr-x    - jr 26 Aug 15:33  dir2
.rw-r--r-- 2.1k jr 26 Aug 15:00  fun
lrwxrwxrwx    - jr 26 Aug 15:32  fun-sym -> fun
```
