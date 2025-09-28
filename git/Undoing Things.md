---
id: Undoing Things
aliases:
  - Undoing Things
tags: []
---

# Undoing Things

At any stage, you may want to undo something.

- One of the common undos takes place when you commit too early and possibly 
forget to add some files, or you mess up the commit message.
- If you want to redo that commit, make the additional changes you forgot,
stage them, and commit them again using the `--amend` option.

```bash
git commit --amend
```

- This command takes your staging area and uses it for the commit.
- If you've made no changes since your last commit, then your snapshot will
look exactly the same, and all you'll change is your commit message.

As an example, if you commit then realize you forgot to stage the changes in a 
file you wanted to add to this commit, you can do something like this:

```bash
git commit -m "Initial commit"
git add forgotten_file
git commit --amend
```
- You end up with a single commit, the second commit replaces the results of the
first.

> :i: It's important to understand that when you're amending your last commit
you're not so much fixing it as *replacing it* entirely with a new, improved
commit that pushes the old commit out of the way and puts the new in its place.

[[Unstaging a Staged File.md]]

