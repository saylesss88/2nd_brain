# Practical Jujutsu

## The Basics

This will assume that you are using JJ with an existing remote repository that
you want to push changes to.

**Initialize a Repository**

Jujutsu works on top of a Git repo, so the first step is to either clone, or for
a new project, run `git init` in the root directory of the project. Once you
have a Git repo, you can run `jj git init --colocate` to enable using both jj
and Git commands.

> ❗ NOTE: You can also use `jj git clone` but there is less direct Git
> interaction. It is designed for primarily using `jj` commands and makes using
> Git commands more complex (e.g., specifying `--git-dir`, and `--work-tree`) If
> you want to use both `git` and `jj` commands, it's recommended to use a
> colocated repository.

## 🔑 Key Terms

**Working copy**: JJ attempts to simplify the working copy by eliminating the
staging area ("index") and instead associating the working copy directly with a
special, mutable commit called the **working commit**(or "current commit" or
"head commit of the working copy"). It is always an editable representation of a
specific commit.

A **change** is a commit that can evolve while keeping a stable identifier.

Your working copy (the `@` symbol) is **always detached** it points directly to
a commit, not a branch or bookmark. When you make a new commit, your `@` moves
forward to a new, empty commit on top of your last change. This is by design,
`jj` never keeps your working copy attached to a branch or bookmark; you always
start fresh on a new mutable commit.

When you run `jj git push`, JJ syncs all bookmarks and their current commit
pointers to the remote. If your local `main` bookmark points to commit `A`, and
you push, the remote's `main` branch will now point to `A` as well. After the
push, your bookmark (pointer) stays pointing at `main` and you are provided a
new working copy with nothing pointing to it. If you run
`jj bookmark set main -r @` you are telling jj to point the `main` bookmark at
your current working copy.

Example of `jj log` after running `jj git push`

```bash
@  abc123 (empty) (no description set)
│
◆ def456 main git_head() ...
```

The working copy is at `abc123`, this is where you can make new changes.

The bookmark `main` is still at `def456`

If you want `main` to mark your latest work, you must run
`jj bookmark set main -r @` which is telling jj to point the bookmark at `-r`
(revision) `@` (working copy).
