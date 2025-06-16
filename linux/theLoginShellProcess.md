---
title: Understanding the login shell process
date: 2024-10-29
tags: [tag1, tag2]
---

# Understanding the login shell process

When you log in to the Linux system, the bash shell starts as a login shell. The
login shell typically looks for five different startup files to process commands
from:

- `/etc/profile`

- `$HOME/.bash_profile`

- `$HOME/.bashrc`

- `$HOME/.bash_login`

- `$HOME/.profile`

The `/etc/profile` file is the main default startup file for the bash shell on
the system. All users on the system execute this file when they log in.

The other 4 files are specific for each user and can be customized for an
individual user's requirements.
