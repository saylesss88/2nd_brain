---
title: Changing Ownership
date: 2024-10-30
tags: [tag1, tag2]
---

# Changing Ownership

The format of the `chown` command is:

`chown options owner[.group] file`

```bash
# chown dan newfile
# ls -l newfile
-rw-rw-r--     1 dan   rich   0 Sep 20 19:16 newfile
```

Or you can change both the user and group of a file:

```bash
# chown dan.shared newfile
# ls -l newfile
-rw-rw-r--      1 dan    shared    0 Sep 20 19:15 newfile
```

If you want to get tricky, you can just change the default group for a file:

```bash
# chown .rich newfile
# ls -l newfile
-rw-rw-r--       1  dan    rich    0 Sep 20 19:15 newfile
```

Finally, if your Linux system uses individual group names that match user login
names, you can change both with just one entry:

```bash
# chown test. newfile
# ls -l newfile
-rw-rw-r--      1 test    test      0 Sep 20 19:15 newfile
```

`chown -R` (recursive changes through subdirs and files, using wildcards)

`-h` parameter also changes the ownership of any files that are symbolically
linked to the file.

> [!NOTE] Only the root user can change the owner of a file. Any user can change
> the default group of a file, but the user must be a member of the groups the
> file is changed from and to.

The `chgrp` command provides an easy way to change just the default group for a
file or directory:

```bash
chgrp shared newfile
ls -l newfile
-rw-rw-r--      1  rich   shared
```

- The user account must own the file, and be a member of the new group as well
  to be able to change the group.

  - Now any member of the shared group can write to the file. This is one way to
    share files on a Linux system. However, sharing files among a group of
    people on the system can get tricky..

[[sharingFiles.md]]
