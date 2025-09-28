---
id: Removing Files
aliases:
  - Removing Files
tags: []
---

# Removing Files

To remove a file from Git, you have to remove it from your tracked files (staging
area) and then commit.

The `git rm` command does that, and also removes the file from your working
directory so you don't see it as an untracked file the next time around.

If you simply remove the file from your working directory, it shows up under the
"Changes not staged for commit" area of your git status output:

```bash
$ rm PROJECTS.md
$ git status
On branch master
Your branch is up-to-date with 'origin/master'.
Changes not staged for commit:
(use "git add/rm <file>..." to update what will be committed)
(use "git checkout -- <file>..." to discard changes in working directory)
deleted: PROJECTS.md
no changes added to commit (use "git add" and/or "git commit -a")
```

Then if you run `git rm`, it stages the file's removal:

```bash
git rm PROJECTS.md
rm 'PROJECTS.md'
$ git status
On branch master
Your branch is up-to-date with 'origin/master'.
Changes to be committed:
  (use "git reset HEAD <file>..." to unstage)

deleted: PROJECTS.md
```

The next time you commit, the file will be gone and no longer tracked. If you
modified the file or had already added it to the staging area, you must force
the removal with the `-f` option.

- This is a safety feature to prevent accidentally removing files that haven't
  been recorded in a snapshot and that can't be recovered from Git.

- Another thing you may want to do is keep the file in your working tree but
  remove it from your staging area. In other words, you may want to keep the file
  on your hard drive but not have Git track it.

- This is particularly useful if you forgot to add something to your .gitignore
  file and accidentally staged it, like a log file or a bunch of .a compiled files.
  To do this, use the `--cached` option:
  `git rm --cached README`

- You can pass files, directories, and file-glob patterns to the `git rm` command
  That means you can do things such as:
  `git rm log/\*.log`

>:Note: the backslash (`\`) in front of the asterisk (`*`). This is necessary because
Git does its own filename expansion in addition to your shell's filename expansion.

This command removes all files that have
the .log extension in the log/ directory. Or, you can do something like this:
`git rm \*~`

This command removes all files that end in ~

[[Moving Files.md]]

