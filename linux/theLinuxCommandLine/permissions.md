---
title: Permissions
date: 2024-10-11
tags: [tag1, tag2]
---

# Permissions

## Reading, Writing, and Executing

Access rights are defined in terms of read access, write access, and execute
access.

If we look at the output of `ls -l`:

```bash
ls -l
```

`drwxr-xr-x - jr 10 Oct 18:46  2nd_brain`

The first 10 characters of the listing are the _file attributes_.

- The first of these is the file type.(- for regular files, `d` for directories)

- The other 9, called the _file mode_, represent the access rights.
