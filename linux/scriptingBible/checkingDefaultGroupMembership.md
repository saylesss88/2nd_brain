---
id: checkingDefaultGroupMembership
aliases:
  - Checking Default Group Membership
tags: []
---

# Checking Default Group Membership

The `-G` comparison checks the default group of a file, and it succeeds if it
matches the group of the default group for the user.

```bash
#!/usr/bin/bash
# check file group test
#
if [ -G $HOME/testing ]
then
  echo "You are in the same group as the file"
else
  echo "The file is not owned by your group"
fi
```
