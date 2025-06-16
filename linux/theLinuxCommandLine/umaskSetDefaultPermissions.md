---
title: umask Set Default Permissions
date: 2024-10-12
tags: [tag1, tag2]
---

# umask Set Default Permissions

The `umask` command controls the default permissions given to a file when it is created.

- It uses octal notation to express a _mask_ of bits to be removed from a file's
  mode attributes.

```bash
rm -f foo.txt
umask
0002
foo.txt
ls -l foo.txt
-rw-rw-r-- 1 me  me 0 2012-03-06 14:53 foo.txt
```

- Here, we ran the `umask` command without any arguments to see the current
  value. It responded with the value of 0002 (a common default).

- The default creation permissions are `777` for directories and `666` for
  files.

With the `-S` option, the mask will be displayed using symbolic notation.

```bash
umask -S
u=rwx,g=rw,o=rx
```
