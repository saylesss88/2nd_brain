# Git tags

Tags are just named pointers to commits. Create one:

There is already one created here but you would check `jj log` and create a tag
on your most recent change:

```bash
jj st
The working copy has no changes.
Working copy  (@) : ysrlmzlt 7d093162 (empty) (no description set)
Parent commit (@-): wmuwoply ea03805e chore: add 'release version' to justfile
```

```bash
jj tag set v0.1.5 --revision ea03805e
jj git push --tags
cargo publish
```

When you publish next, create a new tag `v0.1.6`.

The simple order:

1. Write code, commit, jj git push as normal (many times)

2. When ready to release:
- bump version in Cargo.toml
- update CHANGELOG
- commit
- `jj git push`
- `jj tag set v0.1.6 --revision @-`
- `jj git push --tag v0.1.6`           # This only pushes the tag, not the commit again
- `cargo publish`

That's it. Tags only appear at step 2, once per release.

**One tag per release, on the commit you publish. Everything in between is just
normal commits with no tags involved.**

## Changelog discipline

As you commit, drop one line into [Unreleased] immediately: don't batch it at
release time from `git log`. Your commits are clean enough that you can
basically copy the subject line directly. When you release:

1. Rename [Unreleased] → ## [0.1.6] - date
2. Add a fresh empty [Unreleased] above it
3. Update the reference links at the bottom
4. Commit: chore(release): 0.1.6
5. Run just release 0.1.6
