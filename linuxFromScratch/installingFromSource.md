---
id: installingFromSource
aliases:
  - Installing From Source
tags:
  - linux
  - software
---

# Installing From Source

## Downloading and Security

The usual way to download an archive file is by using a web-browser and clicking
on a “download” button, clicking a link, or right-clicking a link and selecting
“save as”. This downloads the file using the “http” protocol. Some sites instead
make files available via the older “ftp” protocol.

- `wget` or `curl` command-line apps can also be used under unix-like operating
  systemd to fetch a file via http or ftp when the correct URL is known.

- Both the `http` and `ftp` network protocols can be intercepted by criminals or
  other undesirables.

It is therefore a good idea to verify that what you downloaded is what you want.

- Many sites provide an `md5sum` file for each archive file, which holds a
  `checksum` of the contents of the file.

- Always obtain an md5sums file from the original site, never a mirror. And use
  https when possible.

To compute the sum of a single file:

```bash
md5sum file-to-check
```

and "by hand" verify the output of this application against the expected value.

If the software provider provides an md5sums file which has a list of (filename, checksum) pairs, then you can run:

```bash
md5sum -c md5sums-file
```

which will look in your local system for every file listed in the md5sums-file,
compute its checksum, and report an error if it is not the expected value.

### Archive Files

An _archive_ is a single file that contains within it a number of other files.
There are many tools for this, but the process is to append all the individual
files together, and then add an "index" which contains the offset, length, name
and other properties of the original files so that they can be extracted again.

The "tar" application packs multiple files into a single "tar-format" archive
file, which is the most commonly-used format in open-source.

Tar archives remember not only the original names of files, but also their
original Unix "owner id" and file-permissions. By convention, tar-format files
are usually given names ending in ".tar". Tar-format files containing sourcecode
are sometimes referred to as "tarballs".

- The `tar` application doesn't compress the file contents. However source-code
  files do compress very well and network bandwidth is always too slow and
  expensive, so tar files are usually compressed after creation, using a generic
  compression application.

  - The most commonly used compression application is `gzip`, and the resulting
    file is usually given the suffix ".tar.gz" or ".tgz".

  - The `bzip2` compression application is also used, and the resulting file is
    usually given the suffix ".tar.bz2" or ".tbz2".

```bash
# Modern GNU tar options. This works for files compressed
   # with gzip and bzip too
   tar --extract --file filename

   # Same as above
   tar -xf filename

   # Same as above. Leading "-" is optional
   tar xf filename

   # explicitly decompress gzip2-compressed file then
   # pass uncompressed result directly into tar
   gzip -cd filename.tgz | tar xf -

   # same as above, but for bzip2-compressed files
   bunzip2 -cd filename.tar.bz2 | tar xf -
```

- If the "f" is omitted, tar just appears to hang (waits for user input).

> [!WARNING] unpacking a tar file can overwrite local files. By default, files
> are unpacked into a subdirectory of the current directory, which should be
> safe as long as your current directory is something appropriate. However,
> don't trust any instructions that use the following options; they may be
> trying to trick you into modifying important files:

```bash
-C or --directory
-P or --absolute-names
--transform or --xform
```

- Most tar-files are created where unpacking them creates a single subdirectory
  within the current directory, and all other files are placed into that
  directory; eg unpacking a file named "foo-1.2.tar.gz" would create a
  subdirectory named "foo-1.2" and place all the files into that directory.

- Not all tar-files follow this convention unfortunately so its best to check
  with these commands:

```bash
# Modern GNU tar options
tar --list --file filename
# Same as above
tar -tf filename
# Same as above. Leading "-" is optional
tar tf filename

gzip -cd filename | tar tf -
bunzip -cd filename | tar tf -
```

- Occasionally, archive files are in "zip format" The contents of a zipfile can
  be extracted with the `unzip` command.

- Where possible, archive files should be unpacked when logged in as a normal
  user, not as root. This is an extra safety measure to prevent any unintended
  file overwrites.

## Common Files

Unpacked source-code archives usually contain a file named README or INSTALL (or
both) in the top-level directory. You should always read these docs first, as
they give important info about how to compile, install and configure the rest of
the source-code in the downloaded archive.

## Patching

Patches are small changes to the source-code that are applied to the source. The
person who solved the problem may publish their "tweak" in the form of a `sed`
or `awk` command, a shell-script containing multiple sed/awk commands, or a
_patch file_.

- The `sed` tool applies regex to each line in a textfile, and replaces matching
  text with something else.

- `awk` does something similar, but is capable of performing more complex
  transformations of textfiles.

A patchfile is created using the "diff" tool to compare two versions of the same
file and output the differences. The `patch` tool can take the output of `diff`
and apply it to one of the files to "convert it" to the other version.

### Build Systems: configure, make, cmake, etc

Developers go to some effort to make it easy to build and install the software
on a range of different systems. However they can't support every possible
configuration in the world.

In the end, the point of the installation process is to convert the original
source-code into a form that the local computer can execute, and then to place
all the necessary files into the correct locations.(usually PATH). Installing
modules for interpreted languages is slightly different; those files get
installed where the interpreter (eg python or perl) can find them, rather than
in $PATH directly.

### Configure and Make

`make` is the most widely-spread tool used to manage the compilation and
installation of source-code. Make is an application which takes a
configuration-file (called a makefile) that contains a list of rules, most of
the form:

- when TARGET-FILE is older than SOURCE-FILE then SOME-ACTION

The target-file is the end product, or some intermediate stage. The source-file
is the hand-written source-code, or some intermediate artifact. The action is
usually the invocation of a compiler, a linker, or similar which recreates the
target-file.

Unfortunately, although makefile syntax is very powerful it still isn't enough
to be able to handle all the possible ways that computers can be configured.

- Therefore, many software packages come with a shell-script called "configure" and a template makefile named "Makefile.in". The configure script takes a list of configuration options on the command line, and converts the template makefile into a real makefile customised for the local computer and the installers wishes. The installation sequence therefore usually goes like:

```bash
# unpack and read documentation
tar xf filename
cd {directory created by above step}
less README
less INSTALL

# generate customised makefile
./configure {some options ...}

# compile everything in the local directory
make

# update global directories
sudo make install
```
