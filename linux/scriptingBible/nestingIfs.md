---
id: nestingIfs
aliases:
  - Nesting ifs
tags:
  - bash
---

# Nesting ifs

Sometimes, you must check for several situations in your script code. For these
situations, you can nest if-then statements:

To check if a logon name is not in the `/etc/passwd` file and yet a directory
for that user still exists, use a nested if-then statement:

```bash
ls -d /home/NoSuchUser/
/home/NoSuchUser/

#!/usr/bin/bash
# Testing nested ifs

testuser=NoSuchUser

if grep $testuser /etc/passwd
then
   echo "The user $testuser exists on this system."
else
   echo "The user $testuser does NOT exist on this system."
   if ls -d /home/$testuser/
   then
      echo "However, the directory /home/$testuser exists."
   fi
fi
```

```bash
#!/bin/bash
# Testing nested ifs - use elif
#
testuser=NoSuchUser
#
if grep $testuser /etc/passwd
then
    echo "The user $testuser exists on this system."
#
elif ls -d /home/$testuser
then
    echo "The user $testuser does not exist on this system."
    echo "However, $testuser has a directory."
#
fi
```
