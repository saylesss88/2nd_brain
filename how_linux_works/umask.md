---
id: umask
aliases:
date: 2024-09-24
---

# Umask command

The `umask` command is used to set default permissions for files or directories
the user creates.

- It specifies the permissions that the user does not want to be given out to
  the newly created file or directory.

- umask works by doing a Bitwise AND with the bitwise complement(where the bits
  are inverted, i.e. 1 becomes 0 and 0 becomes 1) of the umask.

- The bits which are set in the umask value, refer to the permissions, which are
  not assigned by default, as these values are subtracted from the maximum
  permission for files/directories.

## How to calculate umask value?

**Syntax:**

```bash
$umask
```

**Output:**

`0022`

- Here, the first digit, 0 is called the sticky bit, it is a special security feature.

- The next three digits represent the octal values of the umask for a file or
  directory.

- There are three classes of users in linux:

  - The owner

  - group members

  - everyone else

- If you ignore the sticky bit, the access to the owner is 0, access to the
  group is 2, and access to everyone is 2.

A `umask` value of 0022 is quite common and is considered normal. Here’s what
it means:

- Files: By default, files are created with permissions 666 (read and write for
  everyone). The umask value is subtracted from this default, so 666 - 022 = 644.
  This results in files having permissions 644 (read and write for the owner, and
  read-only for the group and others).

- Directories: Directories are created with default permissions 777 (read, write
  , and execute for everyone). Subtracting the umask value, 777 - 022 = 755,
  results in directories having permissions 755 (read, write, and execute for the
  owner, and read and execute for the group and others) 12.

In summary, a umask of 0022 ensures that new files are not writable by others,
and new directories are not writable or executable by others, which is a good
balance between usability and security.

- In general, use umask 022 if you want everyone to be able to see all
  of the files and directories that you create, and use umask 077 if you don’t.

[[absolutePermissionModes.md]]
