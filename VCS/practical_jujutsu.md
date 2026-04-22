## Practical Jujutsu

- `❯` will indicate a command follows.

- No leading `❯` indicates output.

Let's start by version controlling a Nix development environment:

```bash
❯  pwd
/home/jr/projects/rusty

❯  ls
 flake.lock   flake.nix

❯  jj git init --colocate
Initialized repo in "."
Hint: Running `git clean -xdf` will remove `.jj/`!
```

### The edit workflow

From Steves JJ Tutorial:

1. We create a new change to work on our feature.
2. If we end up doing exactly what we wanted to do, we're done.
3. If we realize we want to break this up into smaller changes, we do it by
   making a new change before the current one, swapping to it, and making that
   change.
4. We then go back to the main change.

With the edit workflow, you typically keep your Working copy `@` at your
existing change meaning it likely won't be empty. When we're done with the
change, we start a new one with a description like:

```bash
❯ jj new -m "Next commit message"
```

Or if we're ready to push, since our changes are where our working copy is, we
move the `main` bookmark to point to the Working copy and push:

```bash
❯ jj bookmark set main -r @

❯ jj git push
```

Our current `status`:

```bash
❯  jj st
Working copy changes:
A flake.lock
A flake.nix
Working copy  (@) : k 4118e144 (no description set)
Parent commit (@-): z 00000000 (empty) (no description set)
```

- `@` will always be our working copy. (Where our current changes are tracked)

- `@-` will always be the parent commit

Let's see the entirety of our history so far:

```bash
❯  jj log
@  k sayls8@proton.me 2026-03-15 09:37:22 4118e144
│  (no description set)
◆  z root() 00000000
```

- Every `jj` repo has a root commit with `zzzzzzzz` `00000000 ` identifiers.
  This is the foundation of the repo. Jujutsu created a second change based on
  top of the empty root commit.

- The diamond `◆` represents an immutable, protected revision.

- We just created this repo and there are already 2 changes with change IDs `k`,
  and `z`, and 2 commits with identifiers `4118e144` and `00000000`

## Bookmarks (Branches)

Since this wasn't an existing Git repo, there are no named branches (bookmarks)
yet.

Let's create a new bookmark pointing to our Working copy:

```bash
❯  jj bookmark create main -r @
Created 1 bookmarks pointing to k 4118e144 main | (no description set)

❯ jj bookmark track main --remote=origin
Started tracking 1 remote bookmarks.
```

- We had to run `jj bookmark track main --remote=origin` to connect our local
  `main` to our remote `origin/main`.

Let's check out what happened here:

```bash
❯  jj st
Working copy changes:
A flake.lock
A flake.nix
Working copy  (@) : k 4118e144 main | (no description set)
Parent commit (@-): z 00000000 (empty) (no description set)

❯  jj log
@  k sayls8@proton.me 2026-03-15 09:37:22 main 4118e144
│  (no description set)
◆  z root() 00000000
```

- Now, our Working copy and `main` are in sync. Our `k` change isn't empty, it
  holds our `flake.nix` and `flake.lock`, let's give it a description:

```bash
❯ jj desc -m "Initialize new project directory with a Nix development environment"
Working copy  (@) now at: k e21fca60 main | Initialize new project directory with a Nix development environment
Parent commit (@-)      : z 00000000 (empty) (no description set)
```

We're done with this change, so let's create a new change based off of this one:

```bash
jj new
Working copy  (@) now at: p e5ea4827 (empty) (no description set)
Parent commit (@-)      : k e21fca60 main | Initialize new project directory with a Nix development environment
```

- Notice that `jj new` moved the Working copy forward but our `main` bookmark
  stayed where it was.

There's not really a reason to have them in sync until we are ready to push our
changes.

Let's make another change:

```bash
❯  touch README.md
❯  jj st
Working copy changes:
A README.md
Working copy  (@) : p bb12f9a8 (no description set)
Parent commit (@-): k e21fca60 main | Initialize new project directory with a Nix development environment
```

- We can see that we added `A` `README.md`, the working copy is no longer empty.

```bash
 jj desc -m "feat: add README to project root"
Working copy  (@) now at: p b878be14 feat: add README to project root
Parent commit (@-)      : k e21fca60 main | Initialize new project directory with a Nix development environment

❯  jj log
@  p saylesss87@proton.me 2026-03-15 10:00:31 b878be14
│  feat: add README to project root
○  k sayls8@proton.me 2026-03-15 09:55:19 main e21fca60
│  Initialize new project directory with a Nix development environment
◆  z root() 00000000
```

We're done with the README, let's push our changes so far:


```bash
❯ jj bookmark set main -r @
Warning: Target revision is empty.
Moved 1 bookmarks to pl c9300e71 main* | (empty) (no description set)
```

Now we can push the changes:

```bash
jj git push
```


### Squash workflow

```bash
❯  jj desc -m "add comments to flake.nix"
Working copy  (@) now at: n e1a65b19 (empty) add comments to flake.nix
Parent commit (@-)      : p 0164d819 main | chore: add README content

  rusty   HEAD
❯  jj new
Working copy  (@) now at: s 1697c780 (empty) (no description set)
Parent commit (@-)      : n e1a65b19 (empty) add comments to flake.nix

  rusty   HEAD
❯  hx flake.nix

  rusty   HEAD [!] 46s
❯  jj
Working copy changes:
M flake.nix
Working copy  (@) : s ac0dd844 (no description set)
Parent commit (@-): n e1a65b19 (empty) add comments to flake.nix
```

Let's squash the changes into the parent commit:

```bash
jj squash
Working copy  (@) now at: t c056c3fe (empty) (no description set)
Parent commit (@-)      : n 0de95cad add comments to flake.nix
```

Our current change is empty so if we wanted to push we would set the bookmark to
`@-`

```bash
❯  jj log
@  t saylesss87@proton.me 2026-03-15 10:37:28 c056c3fe
│  (empty) (no description set)
○  n saylesss87@proton.me 2026-03-15 10:37:28 0de95cad
│  add comments to flake.nix
◆  p saylesss87@proton.me 2026-03-15 10:34:19 main 0164d819
│  chore: add README content
~

  rusty   HEAD
❯  jj bookmark set main -r @-
Moved 1 bookmarks to n 0de95cad main* | add comments to flake.nix

  rusty   HEAD
❯  jj git push
Changes to push to origin:
  Move forward bookmark main from 0164d8199faf to 0de95cad1493
git: Enumerating objects: 5, done.
git: Counting objects: 100% (5/5), done.
git: Delta compression using up to 16 threads
git: Compressing objects: 100% (3/3), done.
git: Writing objects: 100% (3/3), 361 bytes | 361.00 KiB/s, done.
git: Total 3 (delta 2), reused 0 (delta 0), pack-reused 0 (from 0)
remote: Resolving deltas: 100% (2/2), completed with 2 local objects
```
