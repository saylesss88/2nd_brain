---
id: textManipulationPrograms
aliases:
  - textManipulationPrograms
tags: []
---

# textManipulationPrograms

## The general regular expression parser

The name `general regular expression print` (`grep`) sounds intimidating, but
`grep` is just a way to find patterns in files or text.

You can display a list of all regular user accounts by using grep to search for
all lines that contain the text `/home` in the `/etc/passwd` file.

```bash
grep /home /etc/passwd
```

Or you could find all environment variables that begin with HO:

```bash
env | grep ^HO
```

### Remove sections of lines of text (cut)

The cut command can extract fields from a line of text or from files. It is very useful for
parsing system configuration files into easy-to-digest chunks.

The following example lists all home directories of users on your system. This grep
command line pipes a list of regular users from the /etc/passwd file and displays the
sixth field (-f6) as delimited by a colon (-d':'). The hyphen at the end tells cut to read
from standard input (from the pipe).

```bash
grep /home /etc/passwd | cut -d':' -f6 -
/home/chris
/home/joe
```

#### Translate or delete characters (tr)

The `tr` command is a character-based translator that can be used to replace one
character or set of characters with another or to remove a character from a line
of text.

The following example translates all uppercase letters to lowercase letters and displays the
words mixed upper and lower case as a result:

```bash
$ FOO="Mixed UPpEr aNd LoWeR cAsE"
$ echo $FOO | tr [A-Z] [a-z]
mixed upper and lower case
```

[[streamEditor.md]]
