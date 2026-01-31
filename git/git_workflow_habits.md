# Git Workflow Habits

- Always fetch first, then rebase/merge: `git fetch upstream --prune` so you're
  not rebasing onto a stale `main`.

- Before you push, simulate what GitHub will do:
  `git merge --no-commit --no-ff upstream/main` (then `git merge --abort`). That
  tells you if the PR is actually mergeable.

- If you do rebase a PR branch, always push with `--force-with-lease` (never
  plain `--force`) so you don't accidentally clobber remote updates.

---

## During Conflict Resolution

- Know which mode you're in: if `git status` says `MERGING`, you must
  `git add ...` then `git commit` to finish; if it's a rebase, you
  `git rebase --continue`. That one check saves a ton of failing.

- After resolving, run a quick "did I leave markers anywhere?" check:
  `git diff --check` (it flags leftover conflict markers and whitespace issues).

**Ghost Commits**

- If your PR has 3 commits in it, and you rebase, Git tries to apply Commit #1
  (Conflict!) -> you fix it -> Git tries to apply Commit #2 (Conflict again!) ->
  you fix it.
  - You might be fixing the same conflict 3 times in one rebase session becuse
    Git is replaying your history frame-by-frame.

How to end this for good: The "Squash" If you have more than one commit in your
PR, you are fighting a losing battle. Let's collapse your changes into one
single block so you only have to fix the conflict once.

1. Check your commit count: git log --oneline upstream/main..add-nix-book (If
   you see more than one line here, this is your problem.)

2. Squash everything into one:

```Bash
git reset --soft upstream/main
git add .
git commit -m "feat: add nix book"
```

3. Push the "Cleaned" version:

```bash
git push origin add-nix-book --force-with-lease
```

---

## CI/lint sanity

- Separate “merge conflicts” from “CI failed”: a linter error is just another
  change request—fix it with a normal commit and push; no rebase required.
  awesome-lint is meant to fail PRs on style rules, so it’s normal for it to
  block you. ​
- When a tool checks repo metadata (like GitHub topics), run it against the
  upstream repo URL (like you did) to see what CI will evaluate; local runs can
  fail on forks due to missing topics.

**A repeatable checklist**

Use this every time someone says “rebase onto latest main”:

```bash
git fetch upstream --prune
git checkout my-branch

# Prefer rebase if project wants linear history:
git rebase upstream/main

# Or if you already merged once and want less risk, merge:
# git merge upstream/main

# Run tests/linters locally
# (ex: npx awesome-lint@2.2.2 https://github.com/nix-community/awesome-nix)

git status
git push --force-with-lease   # only if you rebased
# otherwise: git push
```
