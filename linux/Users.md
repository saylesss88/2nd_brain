---
id: Users
aliases:
  - Users
tags: []
---

# Users

A user is an entity that can run processes and own files.

  - A user is most often associated with a username; however the kernel doesn't
    manage usernames; instead, it identifies users by numerical user IDs.

  - Users primarily exist to support permissions and boundaries. Every
    user-space process has a user *owner*, and processes are said to be run as
    the owner.

  - The most important user is `root` which may terminate and alter another
    user's processes and access any file on the local system. For this reason,
    root is known as the *superuser*.

  - A person that has root access is an administrator on a traditional Unix
    system.


