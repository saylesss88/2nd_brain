---
id: Buffers
aliases:
  - Buffers
tags: []
---

# Buffers

A buffer is an area of Vim's memory used to hold text read from a file. In addition
an empty buffer with no associated file can be created to allow the entry of text.

The `:e filename` command can edit an existing file or a new file. When a file
is edited, the file is read into a new buffer that holds a temporary copy of the
file. Editing makes changes to the buffer. To save a file, the original file is
replaced by writing the buffer to disk.

The `:new` command creates a new window displaying the contents of a new (empty)
buffer.

To list all buffers, use the `:ls` command which is equivalent to `:buffers`.
Each buffer is assigned a number that is displayed in the first column.

The second column describes the state of the buffer. The different states are
explained at :help :ls. The third column is the file name associated with the buffer.

## Working with buffers

The commands `:bnext` and `:bprev` move to the next or previous buffer can be
shortened to `bn` and `bp` respectively.

The `:b` command accepts either a number or a string argument specifying which
buffer to display. The buffers are numbered as they are created and can be viewed
in a list using :ls or :buffers. Supplying a number after :b displays that buffer
(:b12 displays buffer 12)

### Jumping around

Vim keeps track of all the jumps you make in your buffers. You can go to any place
you jumped from with the Ctrl-O and Ctrl-I command (normal mode). This also works
across buffers.

To toggle between the current and last buffer use the Ctrl-^ (normal mode) command.
(on most keyboards, hold down Ctrl and press the 6 key on the main keyboard).

#### Splits

You can use `:split` and `:vsplit` to divide the current area into two windows with
the same buffer. If you supply an argument then one of the new windows is created
with the argument as a file name used for the buffer in the new window

To gain more control over how the splits are created, you can combine the :vertical,
:leftabove and :rightbelow commands. Also of use are the the :sfind and :sb commands.

**Examples**
`:vertical sb 3`
Create a vertical split with buffer 3 in the new window

`:vertical rightbelow sfind file.txt`
Create a vertical split with file.txt in the new window

`:rightbelow sfind file.txt`
Create a horizontal split and read file.txt into the buffer in the bottom window

**Navigating splits**
Use Ctrl-W followed by one of the `hjkl` movement keys.

##### Managing buffers

Buffers are a convenient way to manage multiple files within a project in Vim.
If you are managing multiple projects, consider opening a separate Vim for each
project. This way each Vim instance contains only related buffers in the buffer
list. This is an advantage over other methods for managing multiple files; such
as a combination of buffers, windows, and tabs.

Here are the essential buffer commands:

`:ls` List the current buffers (including their numbers).
`:b <number>` Display the buffer with the given number.
`:b <partial>` Display the first buffer matching the partial name (or press Tab for name completion).
`:bd` Delete the current buffer; will fail if unsaved (nothing is deleted).
`:bd!` Delete the current buffer; will discard any changes (changes are lost).

There are many plugins for managing buffers, but it is often easier to just use
the above commands. Enter :ls to list the buffers, then (while the list is still
displayed), enter a command like :b12 to display buffer 12 (no space is needed).

[[Vim Jump List.md]]
