---
id: changingPermissionsWithLetters
aliases:
  - changingPermissionsWithLetters
tags: []
---

# changingPermissionsWithLetters

You can also turn file permissions on and off using the plus (+) and minus (-)
signs, respectively, along with letters to indicate what changes and for whom.

- Using letters, for each file you can change permission for the user (u), group
  (g), others (o), and all users (a).

  - What you would change includes the read (r), write (w), and execute (x)
    bits.

For example, start with a file that has all permissions open (rwxrwxrwx). Run
the following chmod commands using minus sign options. The resulting permissions
are shown to the right of each command.

The following chmod command results in this permission: r-xr-xr-x:

```bash
chmod a-w file
```

The following chmod command results in this permission: rwxrwxrw-:

```bash
chmod o-x file
```

The following chmod command results in this permission: rwx------:

```bash
chmod go-rwx file
```

Likewise, the following examples start with all permissions closed (---------). The plus
sign is used with chmod to turn permissions on.

The following chmod command results in this permission: rw-------:

```bash
chmod u+rw files
```

The following chmod command results in this permission: --x--x--x:

```bash
chmod a+x files
```

The following chmod command results in this permission: r-xr-x---:

```bash
chmod ug+rx files
```

> [!NOTE] : Using letters to change permission recursively with chmod generally
> works better than using numbers because you can change bits selectively instead
> of changing all permission bits at once.

For example, if you wanted to remove write permission for "other" without
changing any other permission bits on a set of files and directories:

```bash
chmod -R o-w $HOME/myapps
```

- This example recursively removes write permissions for “other” on any files and directories
  below the myapps directory. If you had used numbers such as 644, execute permission
  would be turned off for directories; using 755, execute permission would be turned on for
  regular files. Using o-w, only one bit is turned off and all other bits are left alone.

[[settingDefaultPermissionsWithUmask.md]]
