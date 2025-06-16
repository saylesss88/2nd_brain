---
id: Brace Expansion
aliases: []
tags:
  - commands
---

# Brace Expansion

With _brace expansion_ you can create multiple text strings from a pattern
containing braces:

```bash
echo Front-{A,B,C}-Back
Front-A-Back Front-B-Back Front-C-Back
```

- Patterns to be brace expanded may contain a leading portion called a
  *preamble *and a trailing portion called a _postscript_.

The brace expression itself
may contain either a comma-separated list of strings or a range of integers
or single characters. The pattern may not contain embedded whitespace.
Here is an example using a range of integers:

```bash
echo Number_{1..5}
Number_1 Number_2 Number_3 Number_4 Number_5
```

Here we get a range of letters in reverse order:

```bash
echo {Z..A}
Z Y X W V U T S R Q P O N M L K J I H G F E D C B A
```

Brace expansions may be nested:

```bash
echo a{A{1,2},B{3,4}}b
aA1b aA2b aB3b aB4b
```

- The most common application is to make lists
  of files or directories to be created

For example, if we were photographers
and had a large collection of images that we wanted to organize by years and
months, the first thing we might do is create a series of directories named in
numeric year-month format. This way, the directory names will sort in chrono-
logical order. We could type out a complete list of directories, but that’s a lot
of work and it’s error prone too. Instead, we could do this:

```bash
[me@linuxbox ~]$ mkdir Pics
[me@linuxbox ~]$ cd Pics
[me@linuxbox Pics]$ mkdir {2009..2011}-0{1..9} {2009..2011}-{10..12}
[me@linuxbox Pics]$ ls
2009-01 2009-07 2010-01 2010-07 2011-01 2011-07
2009-02 2009-08 2010-02 2010-08 2011-02 2011-08
2009-03 2009-09 2010-03 2010-09 2011-03 2011-09
2009-04 2009-10 2010-04 2010-10 2011-04 2011-10
2009-05 2009-11 2010-05 2010-11 2011-05 2011-11
2009-06 2009-12 2010-06 2010-12 2011-06 2011-12
```

Pretty Slick!

[[parameterExpansion.md]]
