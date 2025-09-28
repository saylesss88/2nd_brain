# Practical Git

## The Basics

When you run `git init` in for example `~/flake` directory, Git creates a hidden
`.git` directory inside `~/flake`. This `.git` directory is the actual Git
repository. It contains all the history, commits, branches, tags,
configurations, and other metadata for your project.

You use `git clone` to copy an existing repository, and `git init` for starting
a new one. Actually running `git clone` does an internal `git init` for you, so
it's never necessary to first run `git init`, then `git clone` and it will
likely cause errors if you do so.

The directory `~/flake` itself becomes the **working copy**, all the existing
files within `flake` (and new files you create there) are now potential files to
be **tracked** (staged) by this new Git repository.

Running `git init` doesn't automatically start tracking or versioning existing
files, they start in an **untracked** (unstaged) state.

To bring the files under Git's version control, you need to first:

1. **Stage them**:

```bash
git add .
```

This command tells Git to start tracking all the files in the current directory
(and its subdirectories) and adds them to the **staging area** (also known as
the "index").

2. **Commit them**:

```bash
git commit -m "Initial commit"
```

This command takes the files from the staging area and saves a snapshot of them
into the repository's history as your first commit.

## 🔑 Key Terms

**Repository**: A repository is a database where Git stores all information
about your project's history, changes, branches, and metadata. You interact with
the repository whenever you use commands like `git commit` (to record a new
state) or `git log` (to view the commit history). It is the hidden `.git`
subdirectory within the working copy. You obtain a Git repository either by
turning a local directory into a Git repository with `git init` or by cloning an
existing Git repository from elsewhere with `git clone`.

**Working copy**: When run `git clone`, you obtain a working copy of that
repository and when you run `git init` you create your own. There is no
distinction between the working copy and the central repository they are all
full-fledged Git repositories. The working copy reflects the state of the
repository at a specific commit. Also called the "working directory" or "working
tree", it is the set of files and directories that you see, edit, and work with
directly on disk. When you make changes, add, modify, or delete files you are
changing the working copy, not the repository itself. The working copy gets new
data from a repository via `git checkout` or `git pull`.

When you stage files in the working copy a checksum is computed for each one,
that stores the version of the file in the Git repo (i.e. _blobs_), and adds
that checksum to the staging area (i.e. the "index"). When you run `git commit`,
Git checksums each subdirectory and stores them as a tree object in the Git
repo. This is how Git keeps track of changes, by comparing checksums between
commits.

A **commit**: Records changes to the repository. When you make a commit, you are
creating a snapshot of the current contents of the index, Git stores the commit
object that contains a pointer to the snapshot of the content you staged. (i.e.,
a branch).

```bash
git commit -h
# or
man git commit
```

A **branch** is just a lightweight movable pointer to a **single** commit. There
are **local branches** and **remote branches**. Local branches are branches that
exist in your local repository and remote branches are references to the state
of branches in a remote repository (i.e., GitHub). You push changes from your
local branch to the remote branch to share your work or preserve changes.

Here's a short breakdown:

- **Pointer**: Think of it like a label or sticky note.

- **Movable**: When you make a new commit on a branch, the branch pointer
  automatically moves forward to point to that new commit. (a notable difference
  from JJ VCS where the pointers do not automatically move).

- **Single Commit**: It only ever points to one specific commit in the commit
  history.

```bash
C1 -- C2 -- C3 (main)
       \
        C4 -- C5 -- C6 (feature-branch)
```

- The `main` branch is a pointer to commit `C3`.

- The `feature-branch` is a pointer to commit `C6`.

**What happens when you commit?**

If you are on `feature-branch` and create a new commit `C7`:

```bash
C1 -- C2 -- C3 (main)
       \
        C4 -- C5 -- C6 -- C7 (feature-branch)
```

- The `feature-branch` is now a pointer to commit `C7`.

## Merging and Rebasing Branches

When you work on a project with Git, you often use branches to separate
different lines of development. To bring changes from one branch into another,
you use merge or rebase. Both methods integrate changes but differ in how they
record history and manage the commit graph.

**Example Scenario**

Branch Layout Before Merge/Rebase:

```bash
C1 -- C2 -- C3 (main)
       \
        C4 -- C5 -- C6 (feature)
```

1. **Merging**

**Merging** combines the work from two branches. Git creates a _merge commit_
that connects the histories of both branches, preserving each branche's linear
history as it happened.

**What happens when you merge `feature` into `main`?**

Suppose you switch to `main` and run:

```bash
git checkout main
git merge feature
```

The result:

```bash
C1 -- C2 -- C3 ------ C7 (main)
       \             /
        C4 -- C5 -- C6 (feature)
```

- `C7` is the merge commit, tying together the histories of `main` and
  `feature`.

- All changes from `feature` are now a part of `main`.

- The commit history is **non-linear**, it preserves the branching structure.

**Key Points**

- No commits are lost or rewritten.

- Merge commits serve as milestones that show when a branch was integrated.

- If there are conflicting changes, you resolve **all conflicts once** at the
  merge commit.

- With a Merge you are often on `main` (i.e. `git checkout main`) and merging
  another branch into `main`.

2. **Rebasing**

**Rebasing** moves (or "reapplies") your feature branch commits on top of
another branch, creating a new, linear history. This cleans up the history by
"fast-forwarding" your work as if it was started from the latest state of the
main branch.

**What happens when you rebase `feature` onto `main`?**

Suppose you switch to `feature` and run:

```bash
git checkout main
git pull               # ensure `main` is up to date
git checkout feature
git rebase main
```

> ❗Notice that you rebase while on the `feature` branch and not on `main` which
> is different from what you do for a merge.

The result:

```bash
C1 -- C2 -- C3 -- C4' -- C5' -- C6' (feature)
                      (main)
```

- Commits `C4`, `C5`, and `C6` are replayed on top of `C3` and become `C4'`,
  `C5'`, and `C6'` (they get new commit hashes, **important**).

- The history is **linear** without a merge commit.

- `feature` now appears as if it started from the latest `main`.

- You typically merge (fast-forward) feature into main.

**Key Points**

- Rebasing **rewrites history** by creating new commits.

- Cleaner log, better for a tidy project history.

- If there are conflicting changes, you may need to resolve conflicts multiple
  times (once per commit).

- With a Rebase you are typically on a `feature` branch and rebasing it onto
  `main`.
