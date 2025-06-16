---
id: Unmodify a Modified File
aliases:
  - Unmodify a Modified File
tags: []
---

# Unmodify a Modified File

How can you easily unmodify a file -- revert it back to what it looked like when
you last committed (or initially cloned, ect.). 
- Luckily `git status` tells you how to do that too.

In the last example output, the unstaged area looks like this:

```bash
Changes not staged for commit:
(use "git add <file>..." to update what will be committed)
(use "git checkout -- <file>..." to discard changes in working directory)
modified:
CONTRIBUTING.md
```
It tells you explicitly how to discard the changes you've made:

```bash
$ git checkout -- CONTRIBUTING.md
$ git status
On branch master
Changes to be committed:
(use "git reset HEAD <file>..." to unstage)
renamed:
README.md -> README
```
- You can see that the changes have been reverted.

> It's important to understand that `git checkout -- <file>` is a dangerous
command. Any local changes you made to that file are gone --Git just replaced
that file with the last staged or committed version.

[[Undoing things with git restore.md]]
