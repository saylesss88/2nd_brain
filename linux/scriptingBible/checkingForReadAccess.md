---
id: checkingForReadAccess
aliases:
  - Checking for read access
tags:
  - bash
  - scripting
---

# Checking for read access

Before trying to read data from a file, it's usually a good idea to check that
the user has read access to the file first. You do this with the `-r`
comparison:

```bash
#!/usr/bin/bash
# testing if you can read a file
pwfile=/etc/shadow
#
# first, test if the file exists, and is a file
if [ -f $pwfile ]
then
    # now test if you can read it
    if [ -r $pwfile ]
    then
        tail $pwfile
    else
        echo "Sorry, $pwfile cannot be read."
    fi
else
    echo "Sorry, $pwfile does not exist."
fi
```

The /home/jr/sentinel file exists, but is empty.
Deleting empty file...

## Checking for write access

The `-w` comparison tests if you can write to a file.
