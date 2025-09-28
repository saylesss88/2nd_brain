---
title: parameter expansion
date: 2024-10-11
tags: [tag1, tag2]
---

Parameter expansion is variables that are used in shell scripts.

To see a list of available variabled:

```bash
printenv | less
```

If you mistype it expands to nothing and returns an empty string.

## Command Substitution

Command Substitution allows us to use the output of a command as an expansion:

```bash
echo $(ls)
```

```bash
ls -l $(which cp)
```

Output:
`.rwxr-xr-x 113k root 30 Aug 07:57  /usr/bin/cp`

- Here we passed the results of `which cp` as an argument to the `ls` command,
  thereby getting a listing of the cp program without having to know its full
  pathname.

There is an alternative syntax for command substitution in older shell programs
that is also supported in bash. It uses _back quotes_ instead of the dollar sign
and parentheses:

```bash
ls -l `which cp`
```

[[quoting.md]]
