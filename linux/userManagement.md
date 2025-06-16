---
id: User Management
aliases:
  - User Management
tags: []
---

# User Management

## What is a User?

A user account provides separation between different people and programs that
can run commands.

- Systems identify users by a unique number called user ID (UID).
- User accounts form the foundations of system security.

### Three Main Types of User Accounts:

1. Superuser: The superuser has complete access to the system. The name of the
   superuser is root. It has a UID of 0.
2. System user: The system user has limited access to the system. It has a UID
   of 1.
3. Regular user: Human users who have access to the system.

The `id` command displays the user ID (UID) of the current user.

```bash
$ id
uid=1000(jr) gid=1000(jr) groups=1000(jr)
```

To view user-related info for processes, use the `ps` command with the `-u` flag.

```bash
ps -u
# Output
USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         1  0.0  0.1  16968  3920 ?        Ss   18:45   0:00 /sbin/init splash
root         2  0.0  0.0      0     0 ?        S    18:45   0:00 [kthreadd]
```

- By default, systems use the `/etc/passwd` file to store user information.

The /etc/passwd file contains the following information about each user:

    Username: root – The username of the user account.

    Password: x – The password in encrypted format for the user account that is stored in the /etc/shadow file for security reasons.

    User ID (UID): 0 – The unique numerical identifier for the user account.

    Group ID (GID): 0 – The primary group identifier for the user account.

    User Info: root – The real name for the user account.

    Home directory: /root – The home directory for the user account.

    Shell: /bin/bash – The default shell for the user account. A system user might use /sbin/nologin if interactive logins are not allowed for that user.
