---
title: Using Temp Files
date: 2024-11-27
tags: [bash, tag2]
---

# Using Temp Files

Linux uses the `/tmp` directory for files that don't need to be kept
indefinitely. Most distros configure the system to automatically remove any
files in the `/tmp` directory at bootup.

- Any user account has privileges to read and write files in the `/tmp`
  directory. This feature provides an easy way for you to create temp files that
  you don't have to worry about cleaning up.

- The `mktemp` command allows you to easily create a unique temp file in the
  `/tmp` folder. The shell creates the file but doesn't use your default umask
  value. Instead, it only assigns read and write permissions to the file's owner
  and makes you the owner of the file.
  - After you create the file, you have full access to read and write to and
    from it from your script, but no one else can access it.

## Creating a local temporary file

By default, mktemp creates a ﬁle in the local directory. To create a temporary ﬁle in a local
directory with the mktemp command, you just need to specify a ﬁlename template. The
template consists of any text ﬁlename, plus six X’s appended to the end of the ﬁlename:

```bash
mktemp testing.XXXXXX
```

- The `mktemp` command replaces the six X’s with random characters.

- You can create multiple temp files and be assured that each one is unique:

```bash
$ mktemp testing.XXXXXX
testing.1DRLuV
$ mktemp testing.XXXXXX
testing.lVBtkW
$ mktemp testing.XXXXXX
testing.PgqNKG
$ ls -l testing*
-rw-------
1 rich
-rw-------
1 rich
-rw-------
1 rich
-rw-------
1 rich
$
rich
rich
rich
rich
0 Oct 17 21:57 testing.1DRLuV
0 Oct 17 21:57 testing.PgqNKG
0 Oct 17 21:30 testing.UfIi13
0 Oct 17 21:57 testing.lVBtkW
```
