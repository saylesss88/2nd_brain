---
id: Checking the Status of Your Files
aliases:
  - Checking the Status of Your Files
tags: []
---

# Checking the Status of Your Files

The main tool you use to determine which files are in which state is the `git 
status` command. If you run it directly after a clone, you should see something
like this:

```bash
$ git status
On branch master
Your branch is up to date with 'origin/master'.
nothing to commit, working tree clean
```

- This means you have a clean working directory; in other words, none of your
tracked files are modified.
- Git also doesn't see any untracked files, or they would be listed here.
- Finally, the command tells you which branch you're on, and informs you that
it has not diverged from the same brance on the server. For now that branch is
always `master` or `main`.

Let’s say you add a new file to your project, a simple README file. If the file
didn’t exist before, and you run git status, you see your untracked file like so:

```bash
$ echo 'My Project' > README
$ git status
On branch master
Your branch is up-to-date with 'origin/master'.
Untracked files:
(use "git add <file>..." to include in what will be committed)

README

nothing added to commit but untracked files present (use "git add" to track)
```
- You can now see that your `README`file is untracked.
- Git won't start including it in your commit snapshots until you explicitly
tell it to do so.

```bash
git add README
git status
On branch master
Your branch is up-to-date with 'origin/master'.
Changes to be committed:
  (use "git restore --staged <file>..." to unstage)

  new file:   README
```

