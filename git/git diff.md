---
id: git diff
aliases:
  - git diff
tags: []
---

# git diff

To see what you've changed but not yet staged, type `git diff` with no other
arguments:

```bash
$ git diff
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
index 8ebb991..643e24f 100644
--- a/CONTRIBUTING.md
+++ b/CONTRIBUTING.md
@@ -65,7 +65,8 @@ branch directly, things can get messy.
Please include a nice description of your changes when you submit your PR;
if we have to read the whole diff to figure out why you're contributing
in the first place, you're less likely to get feedback and have your change
-merged in.
+merged in. Also, split your changes into comprehensive chunks if your patch is
+longer than a dozen lines.
If you are starting to work on a particular area, feel free to submit a PR
that highlights your work in progress (and note in the PR title that it's
```

That command compares what is in your working directory with what is in your
staging area. The result tells you the changes you've made that you haven't yet
staged.

If you want to see what you’ve staged that will go into your next commit, you
can use git diff --staged. This command compares your staged changes to your
last commit:

```bash
$ git diff --staged
diff --git a/README b/README
new file mode 100644
index 0000000..03902a1
--- /dev/null
+++ b/README
@@ -0,0 +1 @@
+My Project
```

- [>] NOTE: It's important to note that `git diff` by itself doesn't show all changes
  made since your last commit -- only changes that are still unstaged. If you've
  staged all of your changes, `git diff` will give you no output.

 For another example, if you stage the CONTRIBUTING.md file and then edit it,
 you can use git diff to see the changes in the file that are staged and the
 changes that are unstaged. If our environment looks like this: 

```bash
$ git add CONTRIBUTING.md
$ echo '# test line' >> CONTRIBUTING.md
$ git status
On branch master
Your branch is up-to-date with 'origin/master'.
Changes to be committed:
(use "git reset HEAD <file>..." to unstage)
modified:
CONTRIBUTING.md
Changes not staged for commit:
(use "git add <file>..." to update what will be committed)
(use "git checkout -- <file>..." to discard changes in working directory)
modified:
CONTRIBUTING.md
```
Now you can use `git diff` to see what is still unstaged:

```bash
$ git diff
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
index 643e24f..87f08c8 100644
--- a/CONTRIBUTING.md
+++ b/CONTRIBUTING.md
@@ -119,3 +119,4 @@ at the
## Starter Projects
See our [projects
list](https://github.com/libgit2/libgit2/blob/development/PROJECTS.md).
+# test line
```
and `git diff --cached` to see what you've staged so far (--staged and --cached
are synonyms):

```bash
$ git diff --cached
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
index 8ebb991..643e24f 100644
--- a/CONTRIBUTING.md
+++ b/CONTRIBUTING.md
@@ -65,7 +65,8 @@ branch directly, things can get messy.
Please include a nice description of your changes when you submit your PR;
if we have to read the whole diff to figure out why you're contributing
in the first place, you're less likely to get feedback and have your change
-merged in.
+merged in. Also, split your changes into comprehensive chunks if your patch is
+longer than a dozen lines.
```

Git Diff in an External Tool
We will continue to use the git diff command in various ways throughout the rest

of the book. There is another way to look at these diffs if you prefer a graphical or
external diff viewing program instead. If you run git difftool instead of git diff,
you can view any of these diffs in software like emerge, vimdiff and many more
(including commercial products). Run git difftool --tool-help to see what is
available on your system.

[[Committing Your Changes.md]]
