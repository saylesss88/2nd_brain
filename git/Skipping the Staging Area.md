---
id: Skipping the Staging Area
aliases:
  - Skipping the Staging Area
tags: []
---

# Skipping the Staging Area

Sometimes the staging area is a bit more complex than you need in your workflow.

- If you want to skip the staging area, add the `-a` flag to the `git commit`
  command. This makes Git automatically stage every file that is already 
  tracked before doing the commit, letting you skip the `git add` step.

```bash
git status
On branch master
Your branch is up-to-date with 'origin/master'.
Changes not staged for commit:
(use "git add <file>..." to update what will be committed)
(use "git checkout -- <file>..." to discard changes in working directory)
  modified:  CONTRIBUTING.md

no changes added to commit (use "git add" and/or "git commit -a")
$ git commit -a -m 'Add new benchmarks'
[master 83e38c7] Add new benchmarks
1 file changed, 1 insertion(+), 0 deletions(-)
```
> :Notice: how you don't have to run `git add` on the `CONTRIBUTING.md` file
in this case before you commit. That's because the `-a` flag includes all
changed files.

[[Removing Files.md]]


