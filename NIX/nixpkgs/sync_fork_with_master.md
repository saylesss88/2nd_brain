## Sync fork with master

1. Initial Setup

```bash
# Add nixpkgs as 'upstream'
git remote add upstream https://github.com/NixOS/nixpkgs.git

# Verify your remotes
git remote -v
```

2. The Clean Sync Workflow

1. Fetch the latest:

```bash
git fetch upstream
```

2. Reset your master:

```bash
git checkout master
git reset --hard upstream/master
```

3. Update your fork:

```bash
git push origin master --force
```

> Warning: Only use `--force` on your own fork and branches you aren't sharing
> with others. Since nixpkgs is huge, avoid making local commits directly to
> your `master` branch, keep your work in separate feature branches.

---

3. Syncing your Feature/PR Branches

```bash
git checkout my-new-package
git rebase master
# If there are conflicts, fix them, then:
# git add .
# git rebase --continue
```
