---
title: Creating New Groups
date: 2024-10-30
tags: [bash, tag2]
---

# Creating New Groups

The `groupadd` command allows you to create new groups on your system.

When you create a group, no users are assigned to it by default. For that you
use the `usermod` command:

```bash
# /usr/sbin/usermod -G shared jr
# /usr/sbin/usermod -G shared test
# tail /etc/group
```

> [!CAUTION] If you use `-g`, the group name you specify replaces the default
> group for the user account. The `-G` parameter adds the group to the list of
> groups the user belongs to, keeping the default group intact.

## Modifying Groups

The `groupmod` command allows you to change the GID (using the `-g` parameter)
or the group name (using the `-n` parameter) of an existing group:

```bash
# /usr/sbin/groupmod -n sharing shared
# tail /etc/group
```
