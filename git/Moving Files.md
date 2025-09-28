---
id: Moving Files
aliases:
  - Moving Files
tags: []
---

# Moving Files

Unlike many other VCSs, Git doesn't explicitly track file movement.

- If you rename a file in Git, no metadata is stored in Git that tells it you
 renamed it. However, Git is pretty smart about figuring that out after the fact.

 Thus it's a bit confusing that Git has a `mv` command. If you want to rename a 
 file in Git, you can run something like:

```bash
git mv file_from file_to
```````````````

And this works fine, if you look at the status:
```bash
$ git mv README.md README
$ git status
On branch master
Your branch is up-to-date with 'origin/master'.
Changes to be committed:
(use "git reset HEAD <file>..." to unstage)
renamed:
README.md -> README
```
However, this is equivalent to:

```bash
git mv README.md README
git rm README.md
git add README
```
The only real difference is that `git mv` is one command instead of three.
More importantly, you can use any tool to rename a file and address the `add/rm`
later, before you commit.

[[Viewing the Commit History.md]]
