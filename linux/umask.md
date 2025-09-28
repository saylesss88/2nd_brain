---
title: umask
date: 2024-10-30
tags: [tag1, tag2]
---

# umask

The umask command sets the default permissions for any ﬁle or directory you create:

```bash
touch newfile
ls -al newfile
-rw-r--r--     1 rich    rich     0 Sep 20 19:14 newfile
```

The first digit represents a special security feature called the sticky bit.

```bash
umask
022
```

The umask value is subtracted from the full permission set for an object. The full permission for a ﬁle is mode 666 (read/write permission for all), but for a directory it’s 777 (read/
write/execute permission for all).
