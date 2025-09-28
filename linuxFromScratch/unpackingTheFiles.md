---
id: unpackingTheFiles
aliases:
  - Unpacking the files
tags: []
---

# Unpacking the files

You have downloaded or otherwise acquired a software package. Most likely it is
archived (tarred) and compressed (gzipped), in `.tar.gz` or `.tgz` form
(familiarly known as a "tarball").

First copy it to a working directory. Then _untar_ or _gunzip_ it with:

```bash
tar xzvf filename
```

Where _filename_ is the name of the software file. The de-archiving process will
usually install the appropriate files in subdirectories it will create.

Note that if a package has a .Z suffix, then the above procedure will work as
well, though running `uncompress`, followed by `tar xvf` also works.

- You may preview this process by a `tar tzvf filename`, which lists the files
  in the archive without actually unpacking them.

The above method of unpacking "tarballs" is equivalent to the following:

- `gzip -cd filename | tar xvf -`

- `gunzip -c filename | tar xvf -`

(The `-` causes the tar command to take its input from standard input.)

- Source files in the new _bzip2_ format can be unarchived by a `bzip2 -cd
filename | tar xvf -`, or `tar xyvf filename`, assuming that tar has been
  appropriately patched.

Sometimes the archived file must be untarred and installed from the user's home
directory, or in a certain other directory, such as /, /usr/src, or /opt, as
specified in the package's config info.

- Occasionally, you may need to update or incorporate bug fixes into the unarchived source files using a patch or diff file that lists the changes. The doc files and/or README file will inform you should this be the case.

[[usingMake.md]]
