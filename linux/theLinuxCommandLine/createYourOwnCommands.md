---
id: Creating Your Own Commands with alias
aliases:
  - Creating Your Own Commands with alias
tags: []
---

# Creating Your Own Commands with alias

You can chain commands together using the semi-colon (`;`) character.

```bash
command1; command2; command3...
```
Example:

```bash
cd /usr; ls; cd -
drwxr-xr-x - root 27 Aug 08:48  bin
drwxr-xr-x - root 27 Aug 07:57  include
drwxr-xr-x - root 27 Aug 07:57  lib
drwxr-xr-x - root 26 Aug 18:18  lib32
lrwxrwxrwx - root  7 Apr 14:02  lib64 -> lib
drwxr-xr-x - root 23 Aug 13:33  local
lrwxrwxrwx - root  7 Apr 14:02  sbin -> bin
drwxr-xr-x - root 26 Aug 15:19  share
drwxr-xr-x - root 26 Aug 10:29  src
```
To create this as an alias:

first check if the name is already taken with:

```bash
type test
type foo
```

if you wanted to name it test. Thats taken though...

foo isn't taken:

```bash
alias foo='cd /usr; ls; cd -'
```

[[Redirection.md]]
