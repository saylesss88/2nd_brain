---
id: findingFiles
aliases:
  - findingFiles
tags: []
---

# findingFiles

To help you find files on your system, you can use `locate` (to find commands by
name), `find` (to find files based on lots of different attributes), and `grep`
(to search within text files to find lines in files that contain search text).

## Using locate to find files by name

On most Linux systems, the `updatedb` command runs once per day to gather the
names of files throughout your system into a database.

- By running the `locate` command, you can search that database to find the
  location of files stored in it.

- A few things to know about searching for files using the `locate` command:

  - There are advantages and disadvantages to using the `locate` command instead
    of the `find` command. A `locate` command finds files faster because it
    searches a database instead of having to scan the entire system. A
    disadvantage is that the `locate` command cannot find files added to the
    system since the previous time the db was updated.

  - Not every file in your filesystem is stored in the database.

[[searchingForFilesWithFind.md]]
