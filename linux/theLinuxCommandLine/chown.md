---
title: chown Change Owner and Group
date: 2024-10-12
tags: [tag1, tag2]
---

# chown Change Owner and Group

The `chown` command is used to change the owner and group of a file or
directory. Superuser privileges are required.

Syntax:

```bash
chown [owner][:[group]] file...
```

`chown` can change the file owner and or the file group owner depending on the
first argument of the command.

Argument Result

bob Changes ownership of the file from its current owner
to bob.

bob:users Changes ownership of the file from its current owner
to user bob and changes te group owner to group _users_

:admins Changes the group owner to group _admins_. The file
owner is unchanged

bob: Change the file owner to bob and group owner to the
login group of bob

[[Processes.md]]
