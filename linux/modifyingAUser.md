---
title: Modifying a user
date: 2024-10-30
tags: [bash, tag2]
---

# Modifying a User

| Command      | Description                                    |
| ------------ | ---------------------------------------------- |
| `usermod`    | Edits user account fields, & group membership  |
| ---------    | ---------------------------------------------  |
| `passwd`     | Changes the password                           |
| ---------    | ---------------------------------------------  |
| `chage`      | Changes the passwd's expiration date           |
| ---------    | ---------------------------------------------- |
| `chfn`       | Changes the user account's comment info        |
| ----------   | ---------------------------------------------- |
| `chsh`       | Changes the user account's default shell       |
| ------------ | ---------------------------------------------- |

## usermod

The `usermod` command provides options for changing most of the fields in the
`/etc/passwd` file.

- You use parameters to make the changes you need, most are the same as the
  `useradd` parameters with a few extra:

- `-l` changes the login name of the user account

- `-L` locks the account so the user can't log in.

- `-p` changes the password for the user account

- `-U` unlocks the account so the user can log in

The `-L` parameter is extra handy, use this to lock an account so a user can't
log in without having to remove the account and the user's data. To return the
account to normal, just use the `-U` parameter.

### passwd and chpasswd

A quick way to change just the password for a user is the `passwd` command:

```bash
# passwd test
```

- If you just use the `passwd` command by itself, it changes your own password.

- Any user can change their own password, but only the root user can change some
  one else's password.

  - The `-e` option is handy to force a user to change the password on the next
    log in. This allows you to set the user's password to a simple value and
    forces them to change it to something harder that they can remember.

If you need to do a mass password change for many users, the `chpasswd` command
can be a lifesaver.

The chpasswd command reads a list of login name
and password pairs (separated by a colon) from the standard input, automatically encrypts
the password, and sets it for the user account. You can also use the redirection command to
redirect a ﬁle of userid:password pairs into the command:

```bash
# chpasswd < users.txt
```

#### chsh, chfn, and chage

```bash
# chfn test
Changing finger information for test.
Name []: Ima Test
Office []: Director of Technology
Office Phone []: (123)555-1234
Home Phone []: (123)555-9876

Finger information changed.
# finger test
Login: test                              Name: Ima Test
Directory: /home/test                    Shell: /bin/csh
Office: Director of Technology           Office Phone: (123)555-1234
Home Phone: (123)555-9876
Never logged in.
No mail.
No Plan.
```

The `chage` command helps you manage the password aging process for user
accounts.

The `chage` date values can be expressed it two ways:

- A date in YYYY-MM-DD format

- A numerical value representing the number of days since Jan 1, 1970

The `chage` command also allows you to set an expiration date for an account.
This allows you to create temporary user accounts that expire automatically on a
set date, without having to remember to delete them.
