---
id: Committing Your Changes
aliases:
  - Committing Your Changes
tags: []
---

# Committing Your Changes

Any file that you haven't staged with `git add` will not be included in your 
commit.

- If you run `git status`, and see that everything is staged, you can commit.

```bash
git commit
```

- Doing so launches your editor of choice.

> :warning: This is set by your shell's EDITOR environment variable. 
This can be set with `git config --global core.editor vim`.

> For an even more explicit reminder of what you've modified, you can pass the 
`-v` option to `git commit`. Doing so puts the diff of your change in the editor.

Alternatively, you can type your commit message inline with the commit command by specifying it
after a `-m` flag, like this:
```bash
$ git commit -m "Story 182: fix benchmarks for speed"
[master 463dc4f] Story 182: fix benchmarks for speed
2 files changed, 2 insertions(+)
 mode 100644 README
```

> You can see that the commit has given some output about itself: which branch
you committed to (`master`), what SHA1 hash you used (`463dc4f`), how many files
were changed, and statistics about lines added and removed in the commit.

[[Skipping the Staging Area.md]]
