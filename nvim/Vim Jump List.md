---
id: Vim Jump List
aliases:
  - Vim Jump List
tags: []
---

# Vim Jump List

Jumps are cursor movements stored in a list called the jumplist.

- When making certain movements, such as jumping to line 42 with `42G`, Vim will
  save it as a "jump" in the jumplist.
- You can list jumps using the `:jumps` command.
- To move backwards through the jump list to an older jump use `<ctrl-o>`
- To move to a newer jump use `<ctrl-i>` You can also press `<tab>`

Many shorter movements, like regular hjkl and those based on motions do not
modify the jumplist. So their movements will not be navigable with <ctrl-o> and
<ctrl-i>.

- Similarly, the colon line number (`:42`) movemend doesn't modify the jumplist.
- Use `<line number>G` instead of `:<line number>` to create a point in your
  jump history.

Examples movements which modify the jump list are:

    /pattern searches and ?pattern searches (forward and backward pattern matching)
    * and # (forward and backward search for the word under the cursor.
    % (jump to a matching enclosing character like paren, brace, bracket, etc)
    Any inter-file navigation like gf

Keep in mind that the Vim jump list is a stack, similar to your browsing history.
If you jump back several points and then create a new jump point in the middle
of your history, then jump list points after that will be cleared. Your next
jumps will continue building the stack.
