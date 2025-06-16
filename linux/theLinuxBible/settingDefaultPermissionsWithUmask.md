---
id: settingDefaultPermissionsWithUmask
aliases:
  - settingDefaultPermissionsWithUmask
tags: []
---

# settingDefaultPermissionsWithUmask

- When you create a file as a regular user, it's given permission `rw-rw-r--` by
  default.

- A directory is given the permission `rwxrwxr-x`.

- For the root user, file and directory permissions are `rw-r--r--` and
  `rwxr-xr-x`.

  - These default values are determined by the value of `umask`.

Enter `umask` to see what your umask value is:

```bash
umask
0002
0022  # my value
```

- If you ignore the leading zero for the moment, the `umask` value of 002 results in permissions for a directory of 775 (rwxrwxr-x) and a file of 644 (rw-rw-r--).(Execute permissions are off by default for regular files.)

- To change the `umask` value temporarily run the `umask` command with your
  settings. For example:

```bash
umask 777 ; touch file01 ; mkdir dir01 ; ls -ld file01 dir01
d---------. 2 joe joe 6 Dec 19 11:03 dir01
----------. 1 joe joe 0 Dec 19 11:02 file01

umask 000 ; touch file02 ; mkdir dir02 ; ls -ld file02 dir02
drwxrwxrwx. 2 joe joe 6 Dec 19 11:00 dir02/
-rw-rw-rw-. 1 joe joe 0 Dec 19 10:59 file02

umask 022 ; touch file03 ; mkdir dir03 ; ls -ld file03 dir03
drwxr-xr-x. 2 joe joe 6 Dec 19 11:07 dir03
-rw-r--r--. 1 joe joe 0 Dec 19 11:07 file03
```

- To change the `umask` value permanently, add it to your `~/.bashrc` file.
