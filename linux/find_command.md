---
id: find_command
aliases: []
tags: []
---

# Locating Files and Folders Using the `find` Command

`find` command syntax:

```bash
find /path/ -type f -name file-to-search
```

- `/path` is the path where the file is expected to be found. This is the
  starting point for searching files. The path can also be `/` or `.`

- `-type` represents the file descriptors. They can be any of the below:
`f` – Regular file such as text files, images, and hidden files.
`d` – Directory. These are the folders under consideration.
`l` – Symbolic link. Symbolic links point to files and are similar to shortcuts.
`c` – Character devices. Files that are used to access character devices are
called character device files. Drivers communicate with character devices by
sending and receiving single characters (bytes, octets). Examples include
keyboards, sound cards, and the mouse.
`b` – Block devices. Files that are used to access block devices are called
block device files. Drivers communicate with block devices by sending and
receiving entire blocks of data. Examples include USB and CD-ROM

## How to search files by name or extension

Suppose we need to find files that contain "style" in their name:

```bash
find . -type f -name "style*"
# output
./style.css
./styles.css
```
If you want the "html" extension use:

```bash
find . -type f -name "*.html"
# output
./services.html
./blob.html
./index.html
```

### How to search for hidden files

We can modify the `find` command to search for hidden files:

```bash
find . -type f -name ".*"
```

**List and Find hidden files**

```bash
ls -la
# folder contents
total 5
drwxrwxr-x  2 zaira zaira 4096 Mar 26 14:17 .
drwxr-x--- 61 zaira zaira 4096 Mar 26 14:12 ..
-rw-rw-r--  1 zaira zaira    0 Mar 26 14:17 .bash_history
-rw-rw-r--  1 zaira zaira    0 Mar 26 14:17 .bash_logout
-rw-rw-r--  1 zaira zaira    0 Mar 26 14:17 .bashrc

find . -type f -name ".*"
# find output
./.bash_logout
./.bashrc
./.bash_history
```

Above you see a list of hidden files in my home directory.

### How to search log files and configuration files

Log files usually have the extension `.log` and we can find them like this:

```bash
find . -type f -name "*.log"
```

& for config files

```bash
find . -type f -name "*.conf"
```

[[Displaying the last part of files using tail.md]]
