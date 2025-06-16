---
id: filePermissions
aliases:
  - filePermissions
tags: []
---

# filePermissions

Permissions define the access that you and others have to your files.

- Permission bits for a regular file appear as `-rwxrwxrwx`.

  - Those bits define who can read, write, or execute the file.

> [!NOTE] : For a regular file, a dash is the first bit. For a directory, the
> first bit will be a `d`. symlinks are represented with the `l` bit, `b` for
> block device, `c` for character device, `s` for socket, and `p` for a named
> pipe.

- Of the nine-bit permissions, the first three bits apply to the owner's
  permission, the next three apply to the group assigned to the file, and the
  last three apply to all others.

- The `r` stands for read, `w` for write, and `x` for execute permissions.
- If a dash (`-`) appears in the file's permission bits, it means that the
  permission is turned off for that associated read, write, or execute bit.

## Setting Read, Write, and Execute Permissions

## **Permission** **File** **Directory**

Read | View whats in the file | See what files & subdirs it
contains

---

## Write | Change the files content | Add files and subdirs

Execute | Execute the file | Change to the directory as the
current dir, search through dir,
or execute

---

- You can see the permission for any file or directory by typing the `ls -ld`
  command. The named file or dir appears as those shown here:

```bash
$ ls -ld ch3 test
-rw-rw-r-- 1 joe sales 4983 Jan 18 22:13 ch3
drwxr-xr-x 2 joe sales 1024 Jan 24 13:47 test
```

- The first line shows that the `ch3` file has read and write permissions for
  the owner and the group.

- All other users have read permission, which means that they can view the file
  but cannot change its contents or remove it.

- The second line shows that the `test` directory (`d`). The owner has read,
  write, and execute permissions.

  - As a result, the owner can add, change, or delet files in that directory,
    and everyone else can only read the contents

[[[changingPermissionsWithChmodNumbers.md]]]
