---
title: useradd
date: 2024-10-30
tags: [bash, tag2]
---

# useradd

The primary tool used to add new users is `useradd`.

- The system defaults are set in the `/etc/default/useradd` file.

To see the system default values use the `-D` parameter:

```bash
sudo /usr/sbin/useradd -D
GROUP=984
GROUPS=
HOME=/home
INACTIVE=-1
EXPIRE=
SHELL=/usr/bin/bash
SKEL=/etc/skel
USRSKEL=/usr/etc/skel
CREATE_MAIL_SPOOL=no
LOG_INIT=yes
```

- The useradd command allows an administrator to
  create a default HOME directory conﬁguration and then uses that as a template to create the
  new user’s HOME directory. This allows you to place default ﬁles for the system in every new
  user’s HOME directory automatically. In the Ubuntu Linux system, the /etc/skel directory
  has the following files:

```bash
ls -al /etc/skel
```

By default, the `useradd` command doesn't create a HOME directory, but the `-m`
command line option tells it to create one.

```bash
# useradd -m test
# ls -al /home/test
```

- You can change the system default new user values using the `-D` parameter
  along with a parameter representing the value you need to change.

```bash
# useradd -D -s /usr/bin/zsh
# useradd -D
GROUP=100
HOME=/home
INACTIVE=-1
EXPIRE=
SHELL=usr/bin/zsh
SKEL=/etc/skel
CREATE_MAIL_SPOOL=yes
```

## Removing a User

The `userdel` command removes only the user information from the `/etc/passwd`
file. It doesn't remove any files the account owns on the system.

- If you use the `-r` parameter, `userdel` removes the user's HOME directory,
  along with the user's mail directory.

```bash
# /usr/sbin/userdel -r test
# ls -al /home/test
ls: cannot access /home/test: No such file or directory
```

[[modifyingAUser.md]]
