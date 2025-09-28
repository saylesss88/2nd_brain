---
title: Archiving and compressing files
date: 2024-09-24
tags: [tag1, tag2]
---

## Archiving and compressing files

**gzip**
The program `gzip` (GNU Zip) is one of the current standard Unix compression
programs. A file that ends with `.gz` is a GNU Zip archive.

- Use `gunzip file.gz` to uncompress `<file>.gz` and remove the suffix; to
  compress the file again, use `gzip file`.

**tar**
Unlike the ZIP programs for other operating systems, `gzip` does not create
archives of files; that is, it doesn't pack multiple files and directories into
a single file. To create an archive, use `tar` instead:

```bash
tar cvf archive.tar file1 file2 ...
```

- Archives created by tar usually have a `.tar` suffix.(by convention not
  required).

The c flag activates create mode. The v and f flags have more
specific roles.
The v flag activates verbose diagnostic output, causing tar to print the
names of the files and directories in the archive when it encounters them.
Adding another v causes tar to print details such as file size and permis-
sions. If you don’t want tar to tell you what it’s doing, omit the v flag.
The f flag denotes the file option. The next argument on the command
line after the f flag must be the archive file for tar to create (in the preced-
ing example, it is <archive>.tar). You must use this option followed by a file-
name at all times, except with tape drives. To use standard input or output,
set the filename to a dash ( -).

[[unpackingtarFiles.md]]
