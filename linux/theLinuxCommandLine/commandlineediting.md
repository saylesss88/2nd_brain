---
title: Command Line Editing
date: 2024-10-11
tags: [tag1, tag2]
---

Bash uses a library (a shared collection of routines that different programs can
use) called _Readline_ to implement command line editing.

**Cursor Movement**

`CTRL-A` moves to the beginning of the line.

`CTRL-E` moves to the end of the line.

`CTRL-F` moves forward one character.

`CTRL-B` moves backward one character.

`ALT-F `moves forward one word.

`ALT-B `moves backward one word.

`CTRL-L` clears the screen.

**Editing**

`CTRL-D` deletes the character under the cursor.

`CTRL-T` Transpose (exchange) the character at the cursor location with the one
preceding it.

`ALT-T` Transpose (exchange) the character at the cursor location with the one
preceding it.

`ALT-L` Convert the characters from the cursor to the end of the word to
lowercase.

`ALT-U` Convert the characters from the cursor to the end of the word to
uppercase.

**Cut and Paste Commands**

`CTRL-K` Deletes from the cursor to the end of the line.

`CTRL-U` Deletes from the cursor to the beginning of the line.

`ALT-D` Deletes from the cursor to the end of the word.

`ALT-BACKSPACE` Deletes from the cursor to the beginning of the word.

`CTRL-Y` Yank text from the kill-ring and insert it at the cursor location.

- The Meta Key - `ALT` Since the developers of Readline could not be
  sure of the presence of a dedicated extra control key, they invented one and
  called it meta. While the ALT key serves as the meta key on modern keyboards,
  you can also press and release the ESC key to get the same effect as holding
  down the ALT key if you’re still using a terminal (which you can still do in
  Linux!).

  [[completion.md]]
