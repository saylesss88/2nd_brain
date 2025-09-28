---
id: Git Snapshots
aliases:
  - Git Snapshots
tags: []
---

# Git Snapshots

- Git thinks of its data like a snapshot of a miniature filesystem.
- With Git, every time you commit, or save the state of your project, Git 
basically takes a picture of what all your files look like at that moment and 
stores a reference to that snapshot.
- To be efficient, if files haven't changed, Git doesn't store the file again,
just a link to the previous identical file it already has stored.
- Git thinks about its data more like a **stream of snapshots**

## Nearly Every Operation is Local

- Most operations in Git need only local files and resources to operate -- 
generally no information is needed from another computer on your network.
- Because you have the entire history of the project right there on your local
disk, most operations seem almost instantaneous.

- This means that there is very little you can't do if you're offline.
- You can work anywhere and just push to your *local copy* (i.e. stage locally)

### Git has Integrity

Everything in Git is checksummed before it is stored and is then referred to by
that checksum.
- This means it's impossible to change the contents of any file or directory 
without Git knowing about it.
- You can't lose info in transit or get file corruption without Git being able
to detect it.
- The mechanism that Git uses for this checksumming is called SHA-1 hash.
- This is a 40-character hexadecimal string (0-9, a-f) and calculated based on
the contents of the file or directory structure in Git.

A SHA-1 hash looks something like this:
```bash
24b9da6552252987aa493b52f8696cd6d3b00373
```
You will see these hash values all over the place in Git because it uses them
so much. In fact, Git stores everything in its database not by file name but by
the hash value of its contents.

[[The Three Stages.md]]
