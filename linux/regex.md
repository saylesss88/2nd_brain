---
title: What are Regular Expressions?
date: 2024-09-21
---

# What are Regular Expressions?

A regular expression can be defined as strings that represent several sequences
of characters. One of the most important things about regular expressions is
that they allow you to filter the output of a command or file, edit a section of
a text or configuration file, and so on.

Regular expressions are made of:

- Ordinary characters such as space, underscore(\_), A-Z, a-z, 0-9.

- Meta characters that are expanded to ordinary characters, include:

  - (.) it matches any single character except a newline.

  - (\*) it matches zero or more existences of the immediate character preceding it.

  - [ character(s) ] it matches any one of the characters specified in character(s), one can also use a hyphen (-) to mean a range of characters such as [a-f], [1-5], and so on.

  - ^ it matches the beginning of a line in a file.

  - $ matches the end of the line in a file.

  - \ it is an escape character.
