---
id: understandingSysAdmin
aliases:
  - Understanding System Administration
tags: []
---

# Understanding System Administration

The person assigned to manage all of a Linux system's resources is called a
`system administrator`.

- Even if you are the only person using a Linux system, system administration is
  still set up to be separate from other computer use.

  - To do most administrative tasks, you need to be logged in as the root user
    or use `sudo`.

If you are the system administrator of a Linux system, you generally log in as a regular
user account and then ask for administrative privileges when you need them. This is often
done with one of the following:

- `su` command: Often, `su` is used to open a shell as root user. After the shell is open,
  the administrator can run multiple commands and then exit to return to a shell as a
  regular user.

- `sudo` command: With `sudo`, a regular user is given root privileges, but only
  when that user runs the `sudo` command to run another command. After running that one
  command with sudo, the user is immediately returned to a shell and acts as the
  regular user again.

