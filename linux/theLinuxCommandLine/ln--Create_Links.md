---
id: ln--Create_Links
aliases:
  - ln--Create_Links
tags: []
---

# ln--Create_Links

The `ln` command is used to create either hard or symbolic links. It is used in
one of two ways:

  `ln file link`

to create a hard link and 

  `ln -s item link`

to create a symbolic link where item is either a file or directory.

## Hard Links

Hard links are the original Unix way of creating links; symbolic links are more
modern. 

- By default, every file has a single hard link that gives the file its name.

    - When you create a hard link, you create an additional directory entry for
      a file.

    - Hard links have two important limitations:

        - A hard link cannot reference a file outside its own filesystem. This
        means a link cannot reference a file that is not on the same disk 
        partition as the link itself.

        - A hard link cannot reference a directory.

A hard link is indistinguishable from the file itself. Unlike a directory list
containing a symboolic link, a directory list containing a hard link shows no
special indication of the link.

- When a hard link is deleted, the link is removed, but the contents of the file
  itself continue to exist (that is, the space is not deallocated) until all
  links to the file are deleted.

### Symbolic Links

Symbolic links were created to overcome the limitations of hard links. Symbolic
links work by creating a special type of file that contains a text pointer to
the referenced file or directory.

- A file pointed to by a symbolic link and the symbolic link itself are largely
  indistinguishable. 

    - For example, if you write something to the symbolic link, the referenced
      file is also written to.

    - However, if you delete a symbolic link, only the link is deleted, not the
      file itself.

    - If the file is deleted before the symbolic link, the symbolic link will
      continue to exist but will point to nothing.

    - In this case the link is said to be *broken*.

```bash
cp /etc/passwd .
```

copies the file /etc/passwd to the current directory.

```bash
mv passwd fun
```

renames the file passwd to fun.

```bash
mkdir dir1 dir2
```

```bash
mv fun dir1
```

moves the file fun to dir1.

```bash
mv dir1/fun dir2
```

moves the file fun to dir2.
```bash
mv dir2/fun .
```
moves fun back to the current directory.

[[Creating Hard Links.md]]
