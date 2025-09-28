---
title: Completion
date: 2024-10-11
tags: [tag1, tag2]
---

# Completion

The most common use of completion is the completion of pathnames.

- Completion will also work on variables (if the beginning of the word is a $),
  usernames (if the word begins with ~), commands (if the word is the first word
  on the line), and hostnames (fi the beginning of the word is @). Hostname
  completion only works for hostnames listed in `/etc/hosts`.

## Completion Commands

`ALT-?` Display list of possible completions. On most systems you can also do
this by typing the `TAB` key a second time.

`ALT-*` Insert all possible completions. This is useful if you want to use more
than one possible match.

### Programmable Completion

Recent versions of bash have a facility called `programmable completion` that
allows you (or more likely your distro provider) to add additional completion
rules. It is possible to add completions for the option list of a command or
match particular file types that an application supports.

Run `set | less` to see the list of programmable completion rules.

[[usingHistory.md]]
