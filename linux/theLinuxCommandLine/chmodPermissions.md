---
title: chmod -- Change File Mode
date: 2024-10-11
tags: [tag1, tag2]
---

# chmod -- Change File Mode

To chande the mode (permissions) of a file, use the `chmod` command.

- Only the files owner or the superuser can change the mode of a file or
  directory.

## Octal Representation

Table 9-4: File Modes in Binary and Octal
OctalBinaryFile Mode
0|000|---
1|001|--x
2|010|-w-

```bash
chmod 600 foo.txt
```

By passing the argument `600`, `foo.txt` will are able to set the permissions of
the owner to read and write while removing all permissions from the group owner
and world.

Though remembering octal-to-binary mapping seems inconvenient, you usually have
to use only a few common ones:

`7(rwx)`

`6(rw-)`

`5(r-x)`

`4(r--)`

`0(---)`

### Symbolic Notation

`u` User

`g` Group

`o` Others

`a` All; combination of u, g, and o

- If no character is specified, `a` is assumed.

  - The operation may be `a+` indicating that a permission is to be added. or
  - `a-` indicating that a permission is to be removed.

`u+x` Add execute permission for the owner

`u-x` Remove execute permission for the owner

`+x` Add execute permission for the owner, group, and world. (== `a+x`)

`o-rw` Remove read and write permission for anyone besides the owner and group
owner.

`go=rw` Set the group owner and anyone besides the owner to read and write. If
either the group owner or world previously had execute permissions, remove them.

`u+x,go=rx` Add execute permission for the owner and set the permissions for the
group and others to read and execute.

- Symbolic notation does offer the advantage of allowing you to set a
  single attribute without disturbing any of the others.

> [!NOTE] --recursive option acts on both files and directories, so it's not as
> useful as you'd hope because you rarely want files and directories to have the
> same permissions.
