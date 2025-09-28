---
id: Remove Files and Directories
aliases:
  - Remove Files and Directories
tags: []
---

# Remove Files and Directories

The `rm` command allows you to remove files and directories.

> [!warning] Unix-like operating systems such as Linux do not have an undelete command.
Once you delete something with rm, it’s gone. Linux assumes you’re smart and
you know what you’re doing.
Be particularly careful with wildcards. Consider this classic example. Let’s
say you want to delete just the HTML files in a directory. To do this, you type:
rm *.html
which is correct, but if you accidentally place a space between the * and the
.html like so:
rm * .html
the rm command will delete all the files in the directory and then complain that
there is no file called .html.
Here is a useful tip: Whenever you use wildcards with rm (besides carefully
checking your typing!), test the wildcard first with ls. This will let you see the
files that will be deleted. Then press the up arrow key to recall the command
and replace the ls with rm.

## rm Options

-i, --interactive    Prompt before deleting each file.

-r, --recursive      Remove directories and their contents recursively. This
                     means that if a directory contains subdirectories, the contents of those will be
                     deleted as well.

-v, --verbose        Verbose mode.

Examples:

```bash
rm fun-hard
ls -l
drwxr-xr-x    - jr 26 Aug 15:32  dir1
lrwxrwxrwx    - jr 26 Aug 17:50  dir1-sym -> dir1
drwxr-xr-x    - jr 26 Aug 15:33  dir2
.rw-r--r-- 2.1k jr 26 Aug 15:00  fun
lrwxrwxrwx    - jr 26 Aug 15:32  fun-sym -> fun
```
The file `fun-hard` is gone and the link count shown for fun is reduced from
four to three.

> [!remember] Most file operations are carried out on the link's target, not the
> link itself. However, `rm` is an exception. When you delete a link, it is the
> link that is deleted, not the target.

[[Working With Commands.md]]
[[ln--Create_Links.md]]
