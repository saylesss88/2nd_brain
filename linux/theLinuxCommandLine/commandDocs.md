---
id: Getting a Command's Documentation
aliases: []
tags: []
---

With this knowledge of what a command is, we can now search for the
documentation available for each kind of command.

**help -- Get Help for Shell Builtins**

bash has a built-in `help` command that displays the documentation for for shell
built-ins.

```bash
help cd
cd: cd [-L|-P] [dir]
Change the current directory to DIR. The variable $HOME is the default DIR.
The variable CDPATH defines the search path for the directory containing DIR.
Alternative directory names in CDPATH are separated by a colon (:). A null
directory name is the same as the current directory, i.e. `.'. If DIR begins
with a slash (/), then CDPATH is not used. If the directory is not found, and
the shell option `cdable_vars' is set, then try the word as a variable name.
If that variable has a value, then cd to the value of that variable. The -P
option says to use the physical directory structure instead of following
symbolic links; the -L option forces symbolic links to be followed.
```

**A note on notation:** When square brackets appear in the description of
a command’s syntax, they indicate optional items. A vertical bar character
indicates mutually exclusive items. An example is the cd command above:
cd [-L|-P] [dir].

## man -- Display a Program's Manual Page

Most executable prorams intended for command-line use provide a formal piece of
documentation called a manual or man page. A special paging program called man
is used to view them, like this:

```bash
man program
```
Man pages, do not usually include examples, and are intended as a reference, not
a tutorial.

```bash
man ls
LS(1)                        User Commands                        LS(1)

NAME
       ls - list directory contents

SYNOPSIS
       ls [OPTION]... [FILE]...

DESCRIPTION
       List  information  about the FILEs (the current directory by de‐
       fault).  Sort entries alphabetically if none  of  -cftuvSUX  nor
       --sort is specified.

       Mandatory  arguments to long options are mandatory for short op‐
       tions too.

       -a, --all
              do not ignore entries starting with .

       -A, --almost-all
              do not list implied . and ..

       --author
              with -l, print the author of each file

       -b, --escape
              print C-style escapes for nongraphic characters

```
- On most Linux systems, man uses `less` to display the manual page.

The “manual” that man displays is broken into sections and covers not
only user commands but also system administration commands, program-
ming interfaces, file formats, and more

### Man Page Organization

**Section**             **Contents**
1.                  User commands

2.                  Programming interfaces for kernel system calls

3.                  Programming interfaces to the C library

4.                  Special files such as device nodes and drivers

5.                  File formats

6.                  Games and amusements such as screensavers

7.                  Miscellaneous

8.                  System administration commands

Sometimes we need to look in a specific section of the manual to find
what we are looking for. This is particularly true if we are looking for a file
format that is also the name of a command. If we don’t specify a section num-
ber, we will always get the first instance of a match, probably in section 1. To
specify a section number, we use man like this:
    `man section search_term`

  For example:

```bash
man 5 passwd
```

### apropos -- Display Appropriate Commands

It is also possible to search the list of man pages for possible matches based
on a search term. Though crude, this approach is sometimes helpful. Here
is an example of a search for man pages using the search term floppy:

```bash
apropos floppy
create_floppy_devices (8) - udev callout to create all possible
floppy device based on the CMOS type
fdformat
 (8) - Low-level formats a floppy disk
floppy
 (8) - format floppy disks
gfloppy
 (1) - a simple floppy formatter for the GNOME
mbadblocks
 (1) - tests a floppy disk, and marks the bad
blocks in the FAT
mformat
 (1) - add an MSDOS filesystem to a low-level
formatted floppy disk
```
The first field in each line of output is the name of the man page, and the
second field shows the section.

> [!NOTE] the `man` command with the `-k` option performs exactly the same
> function as `apropos`.

#### whatis -- Display a Very Brief Description of a Command

```bash
whatis ls
ls
 (1) - list directory contents
```

##### info -- Display a Program's Info Entry

The GNU Project provides an alternative to manpages called *info pages*.

```bash
info ls
Next: dir invocation,  Up: Directory listing

10.1 ‘ls’: List directory contents
==================================

The ‘ls’ program lists information about files (of any type, including
directories).  Options and file arguments can be intermixed arbitrarily\
,
as usual.  Later options override earlier options that are incompatible\
.

   For non-option command-line arguments that are directories, by
default ‘ls’ lists the contents of directories, not recursively, and
omitting files with names beginning with ‘.’.  For other non-option
arguments, by default ‘ls’ lists just the file name.  If no non-option
argument is specified, ‘ls’ operates on the current directory, acting a\
s
if it had been invoked with a single argument of ‘.’.

```
The info program reads info files, which are tree-structured into indi-
vidual nodes, each containing a single topic. Info files contain hyperlinks
that can move you from node to node. A hyperlink can be identified by its
leading asterisk and is activated by placing the cursor upon it and pressing
the ENTER key.

```bash
info coreutils
```
- Gives a lot of useful information about coreutils.

###### README and Other Program Documentation Files

The info program reads info files, which are tree-structured into indi-
vidual nodes, each containing a single topic. Info files contain hyperlinks
that can move you from node to node. A hyperlink can be identified by its
leading asterisk and is activated by placing the cursor upon it and pressing
the ENTER key.

[[Creating Your Own Commands with alias.md]]
