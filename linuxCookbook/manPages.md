---
title: man pages
date: 2024-10-13
tags: [tag1, tag2]
---

# man pages

Graphical Viewers

`Konqueror`

`Yelp`

`Pinfo`

## Understanding man pages

Heeding the standard "RTFM" (read the fine man page) advice, you dig up the
relevant man pages and read them but they don't make much sense to you, now
what?

Learn how man pages are organized, and familiarize yourself with their
conventions for teaching command syntax and options.

### Man Page Sections

1. Executable programs or shell commands

2. System calls

3. Library calls

4. Special files (usually found in /dev)

5. File formats and conventions

6. Games

7. Miscellaneous

8. System Administration commands

9. Nonstandard kernel routines

n. New documentation, which may be moved later.

l. Local documentation, specific to your system

When you see references to numbered man pages, like `grep(1)`, it references the
man grep in section 1. Call it up like this:

```bash
man 1 grep
```

Some man pages are in more than one section. man foo will only display the first one.
You can list all of them with the -f switch:

```bash
man -f man
```

### Finding Appropriate man pages

If you know what you want to do but not the command that does it...

Do a keyword search with `apropos` or `man -k` For example, if you want a
command to count the words in a file, use:

```bash
apropos count words
man -k count words
```

### Finding Lost man Pages

First try searching with `whereis -m`:

```bash
whereis -m cat
cat:/usr/man/man1/cat.1.gz /usr/share/man/man1/cat.1.gz
```

Now you've found the page. Read it with `man`:

```bash
man /usr/man/man1/cat.1.gz
```

If that doesn't work, try rebuilding the man database with `mandb`:

```bash
sudo mandb
# or
locate / cat. | egrep -w 'cat\.[1-9][a-zA-Z]*[.gz]?'
```
