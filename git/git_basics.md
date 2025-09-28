---
id: git_basics
aliases:
  - Initializing a Repository in an Existing Directory
tags: []
---

# Initializing a Repository in an Existing Directory

```bash
cd /home/user/my_project
```

and type:

```bash
git init
```

This creates a new subdirectory named `.git` that contains all of your necessary
repository files — a Git repository skeleton. At this point, nothing in your
project is tracked yet.

If you want to start version-controlling existing files (as opposed to an
empty directory), you should probably begin tracking those files and do an
initial commit.

You can accomplish that with a few
git add commands that specify the files you want to track, followed by a git
commit:

```bash
git add *.c
git add LICENSE
git commit -m 'Initial project version'
```

## Checking the Status of Your Files

The main tool to determine which files are in which state is `git status`:

```bash
$ git status
On branch master
Your branch is up-to-date with 'origin/master'.
nothing to commit, working tree clean
```

This means you have a clean working directory; in other words, none of your
tracked files are modified. Git also doesn’t see any untracked files, or they
would be listed here. Finally, the command tells you which branch you’re on and
informs you that it has not diverged from the same branch on the server.


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

You can see that your new README file is untracked, because it’s under the
“Untracked files” heading in your status output. Untracked basically means that
Git sees a file you didn’t have in the previous snapshot (commit), and which
hasn’t yet been staged; Git won’t start including it in your commit snapshots
until you explicitly tell it to do so.

[[Tracking New Files.md]]
