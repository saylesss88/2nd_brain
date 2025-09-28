---
id: Cloning an Existing Repository
aliases:
  - Cloning an Existing Repository
tags: []
---

# Cloning an Existing Repository

Every version of every file for the history of the project is pulled down by
default when you run `git clone`.

You clone a repo with `git clone <url>` For example, if you want to clone the
Git linkable library called `libgit2`, you can do this:

```bash
git clone https://github.com/libgit2/libgit2
```

That creates a directory named `libgit2`, initializes a `.git` directory inside
it, pulls down all the data for that repo, and checks out a working copy of the 
latest version.

If you want to clone the repo into a directory named something different, you
can specify the new directory name as an additional argument:

```bash
git clone https://github.com/libgit2/libgit2 my_libgit2
```
[[Recording Changes to the Repository.md]]
