---
id: changingPermissionsWithChmodNumbers
aliases:
  - changingPermissionsWithChmodNumbers
tags: []
---

# changingPermissionsWithChmodNumbers

If you own a file, you can use the `chmod` command to change the permission on
it as you please.

- In one method, each permission (read, write, and execute) is assigned a
  number:

  - `r` = 4
  - `w` = 2
  - `x` = 1

- And you use each set's total number to establish the permission.

- To make permissions wide open for yourself as owner, you could set the first
  number to 7 (4+2+1), and then you would give the group and others read-only
  permission by setting both the second and third numbers to 4 (4+0+0), so the
  final number is 744.

Here are some examples of how to change permission on a file (named file) and what the
resulting permission would be:

The following chmod results in `rwxrwxrwx`:

```bash
# chmod 744 file
```

For rwxr-xr-x:

```bash
# chmod 755 file
```

For `rw-r--r--`:

```bash
# chmod 644 file
```

For `---------`:

```bash
# chmod 000 file
```

[[changingPermissionsWithLetters.md]]
