---
id: Unstaging a Staged File
aliases:
  - Unstaging a Staged File
tags: []
---

# Unstaging a Staged File

Let's say you've changed two files and want to commit them as two separate changes,
but you accidentally type `git add *` and stage them both.
How can you unstage one of them? The `git status` command reminds you:

```bash
git add *
git status
On branch master
Changes to be committed:
(use "git reset HEAD <file>..." to unstage)
renamed:
README.md -> README
modified:
CONTRIBUTING.md
```
Right below the "Changes to be committed" line, it says use "git reset HEAD <file>..."
to unstage. So, lets use that advice to unstage the `CONTRIBUTING.md` file:

```bash
$ git reset HEAD CONTRIBUTING.md
Unstaged changes after reset:
M
CONTRIBUTING.md
$ git status
On branch master
Changes to be committed:
(use "git reset HEAD <file>..." to unstage)
renamed:
README.md -> README
Changes not staged for commit:
(use "git add <file>..." to update what will be committed)
(use "git checkout -- <file>..." to discard changes in working directory)
modified:
CONTRIBUTING.md
```
> It’s true that git reset can be a dangerous command, especially if you provide the
`--hard` flag. However, in the scenario described above, the file in your working
directory is not touched, so it’s relatively safe.

[[Unmodify a Modified File.md]]
