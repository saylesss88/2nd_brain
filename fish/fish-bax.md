---
title: fish-bax
date: 2024-10-13
tags: [fish, tag2]
---

# fish-bax

Bax is a POSIX shell execution wrapper for the fish shell. Use it to run bash
utilities, replaying environment changes in fish without leaving the comfort of
your session.

## Installation

```bash
fundle plugin 'hunter-richardson/fish-bax'
```

You need to run a script in bash and want to preserve changes in the
environment, e.g., modifications to the $PATH, exported and unset variables,
and so on. What do you do? Nuke the current session.

```fish
exec bash -c "$commands; exec fish"
```

- This is not a rare pattern. Fork a POSIX shell and run the commands in it and
  inherit the environment in fish.

## Usage

```fish
bax export PYTHON=python3
```

That will set the environment variable `PYTHON` in your session.

```fish
env | string match "PYTHON=*"
PYTHON=python3
```
