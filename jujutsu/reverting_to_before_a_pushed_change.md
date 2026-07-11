# Jujutsu (jj) Notes — Reverting, Immutable Commits, Cleanup

## Undo vs. Rewind

**Undo a change without rewriting history** (safe even after pushing, no force needed):
```
jj backout -r <rev>          # new commit that reverses <rev>'s changes
jj bookmark set main -r @
jj git push
```

**Rewind — move a bookmark back to an earlier commit** (rewrites history, needs force push):
```
jj new <rev>                 # start fresh work on top of <rev>
# or: jj edit <rev>           # edit that commit directly, descendants auto-rebase
jj bookmark set main -r <rev-or-@>
jj git push --force
```

Only force-push like this if you're the sole consumer of the branch — anyone else who fetched the old commits ends up with diverged history.

## Navigating to a specific change

```
jj log                        # find the change id (short prefix is enough, e.g. `ku` → kutppqkx)
jj new <change_id>             # new empty commit on top of it, nothing else touched
jj edit <change_id>            # move into that commit directly to amend it in place
```

`jj edit` is the jj-native way to fix a commit in the middle of a stack — no interactive rebase needed, descendants rebase automatically.

## Watch out: mixing raw git commands mid-workflow

Running `git checkout main` (or other raw git commands) inside a colocated jj/git repo can create/move git refs that jj then imports as bookmarks — this can silently shift what `main` points to. Stick to jj's own commands (`jj new`, `jj edit`, `jj bookmark set`) instead of reaching for git directly.

## Immutable commits

Any commit that's an ancestor of a **tracked remote bookmark** (e.g. `main@origin`) is immutable by default — jj blocks you from rewriting/abandoning it to protect shared history.

To force it anyway (fine solo, riskier on shared branches):
```
jj abandon --ignore-immutable <rev>
```

**Better option when starting over:** don't fight immutability at all. Just build new work on an earlier commit and move the bookmark once ready:
```
jj new <earlier-good-rev>
# ...do the work...
jj bookmark set main -r @
jj git push --force
```
Old commits become unreachable once nothing points at them — no need to abandon or un-immutable them first.

## Conflicts cascade down a whole branch

If a commit early in a stack conflicts, **every descendant inherits the conflict** — each commit is "previous commit + diff," so once a diff can't apply cleanly, nothing built on top of it applies cleanly either. This is expected, not a sign everything is broken.

If you're abandoning the whole line anyway, don't bother resolving conflicts one by one — just abandon the range:
```
jj abandon --ignore-immutable <top-rev>::<bottom-rev>
```
`::` works across merges/diamonds too. Conflicted commits with no bookmark pointing at them are inert — they don't block new work or get pushed.

## Cargo gotcha: `include_str!` across crate/workspace boundaries

- **Flat crate (no workspace):** any subdirectory is just part of the same package. `include_str!("../server/src/main.rs")` works fine at both `cargo check` and `cargo publish` time, and just needs the path listed in `Cargo.toml`'s `include`.
- **Workspace with separate crates:** `cargo publish` only packages files *inside* the crate's own directory. A path like `../../server/...` that reaches outside the crate root is **never included**, no matter what's in `include`/`exclude` — this isn't overridable.
- **Nested `Cargo.toml` gotcha:** if you put a real `Cargo.toml` inside a subdirectory (even just as template/embedded text, not a real crate), cargo treats that whole subdirectory as a separate package boundary and excludes it from the parent package — again regardless of `include`. Fix: rename it to something cargo won't recognize as a manifest, e.g. `Cargo.toml.inc`, and reference that name in `include_str!`.

Always verify before publishing:
```
cargo package --list --allow-dirty -p <crate-name>
```
This shows exactly what will ship — much faster than a failed `cargo publish` round trip.

**Rule of thumb:** if embedded "template" files (server scaffolding, boilerplate, etc.) don't need to be independently compiled/tested, keep them as plain files inside the single crate rather than reaching for a workspace. Workspaces earn their keep when crates have genuinely independent version/dependency graphs — not for bundling static text.
