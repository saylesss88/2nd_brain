---
title: Changing Identities
date: 2024-10-12
tags: [tag1, tag2]
---

# Changing Identities

- Log out and log back in

- Use the `su` command

- Use the `sudo` command

From within your own shell session, the `su` command allows you to assume the
identity of another user and either start a new shell session with that user's
ID or issue a single command as that user.

- The sudo command allows administrators to set up a configuration file
  `/etc/sudoers` and define specific commands that particular users are permitted
  to execute under an assumed identity.

## su -- Run a Shell with Substitute User and Group IDs

The `su` command is used to start a shell as another user. The syntax looks like
this:

```bash
su [-[l]] [user]
```

- If the `-l` option is included, the resulting shell session is a login shell
  for the specified user.

  - This means that the user's environment is loaded and the working directory
    is changed to the user's home directory.

  - The `-l` can be abbreviated as `-`:

  ```bash
  su -
  Password:
  [root@localhost ~]#
  ```

Carry out the commands needed and type `exit` to exit back to your user shell.

It's also possible to execute a single command rather than starting a new
interactive command by using su this way:

```bash
su -c 'command'
```

## sudo -- Run a Command as Another User

The administrator can configure sudo to allow an ordinary user to execute
commands as a different user (usually the superuser) in a very controlled way.

- `sudo` doesn't require the superusers password just their own.

To see what privileges you have, use the `sudo -l` command.
