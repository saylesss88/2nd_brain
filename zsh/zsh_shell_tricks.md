---
id: zsh_shell_tricks
aliases: []
tags: []
---

#ZSH SHELL TRICKS

## Cursor Navigation and Line Editing

**Cursor Movement**

- `Ctrl + A` - Move the cursor to the beginning of the line

- `Ctrl + E` - Move the cursor to the end of the line

- `Ctrl + F` - Move the cursor forward one character

- `Ctrl + B` - Move the cursor backward one character

- `Ctrl + P` - Move the cursor up one line

- `Ctrl + N` - Move the cursor down one line

- `Ctrl + U` - Delete the characters before the cursor

- `Ctrl + K` - Cut from the cursor to the end of the line

- `Ctrl + W` - Cut from the cursor to the beginning of the previous word

- `Ctrl + T` - Swap the last two characters before the cursor

- `Ctrl + L` - Clear the screen

- `Alt + D` - Remove from the cursor to the end of the next word.

**Text Manipulation**

- `Ctrl + Y` - Paste from the clipboard

- `Ctrl + Shift + _` - Undo the last keystroke in the command

- `Ctrl + XT` - Swap the current word with the previous one

- `Ctrl + Q` - Move the current command to the buffer, clear the line for a new
  command.

**Command Editing**
Sometimes, you need more advanced editing capabilities to fine-tune commands.

- `Ctrl + XE` - Edit the current command in the default editor

### Globbing

Globbing allows you to specify patterns that match multiple filenames, enabling
you to work with groups of files, directories, and more efficiently.

**Standard Globbing**
Standard globbing uses wildcards to match filenames based on simple patterns.

- `*.txt` - Match all `.txt` files in the cwd

- `file?.txt` - Match files like `file1.txt`, `file2.txt`, etc. in cwd.

These patterns help you quickly select and operate on groups of files without
typing each filename individually.

**Recursive Globbing**
Recursive globbing extends the standard patterns to search through directories
and subdirectories.

- `**/*.txt` - Match all `.txt` files in the cwd and all subdirectories.

>This powerful feature allows you to search for files across an entire directory
tree, making it easy to find and manipulate files regardless of their location.

**Extended Globbing**

Enable extended globbing with the following command:

```zsh
setopt EXTENDED_GLOB
```
Once enabled, you can use additional globbing pattern matching features:

- `ls *(.)` - List only regular files

- `ls *(/)` - List only directories

- `ls *(-/)` - List only directories and symbolic links to directories.

> Extended globbing lets you refine your file searches with greater precision,
making it easier to filter and list files based on specific criteria.

#### Globbing in Action

- `mv *.txt backup/` - Move all `.txt` files in the cwd to `backup/`

- `rm **/*.log` - Remove all `.log` files in the cwd and all subdirectories

- `cp *.{jpg,png} images/` - Copy all `.jpg` and `.png` files in the cwd to
 the `images/` directory.

[[Command History and Expansion.md]]
