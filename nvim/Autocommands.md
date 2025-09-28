---
id: Autocommands
aliases:
  - Autocommands
tags: []
---

# Autocommands

An autocommand is a Vim command or a Lua function that is
automatically executed whenever one or more events are triggered,
eg., when a file is read or written, or when a window is created.
These are accessible from Lua through the Nvim API.

## Creating autocommands

Autocommands are created using `vim.api.nvim_create_autocmd()`,
which takes two mandatory arguments:

- {event}: a string or table of strings containing the event(s) which should
  trigger the command or function.
- {opts}: a table with keys that control what should happen when the event(s) are triggered.

The most important options are:

- pattern: A string or table of strings containing the autocmd-pattern. Note:
  Environment variable like $HOME and ~ are not automatically expanded; you need
  to explicitly use vim.fn.expand() for this.
- command: A string containing a Vim command.
- callback: A Lua function.

You must specify one and only one of command and callback. If pattern is
omitted, it defaults to pattern = '\*'.
Examples:

```vim
vim.api.nvim_create_autocmd({"BufEnter", "BufWinEnter"}, {
  pattern = {"*.c", "*.h"},
  command = "echo 'Entering a C or C++ file'",
})
-- Same autocommand written with a Lua function instead
vim.api.nvim_create_autocmd({"BufEnter", "BufWinEnter"}, {
  pattern = {"*.c", "*.h"},
  callback = function() print("Entering a C or C++ file") end,
})
-- User event triggered by MyPlugin
vim.api.nvim_create_autocmd("User", {
  pattern = "MyPlugin",
  callback = function() print("My Plugin Works!") end,
})
```

Nvim will always call a Lua function with a single table containing information
about the triggered autocommand. The most useful keys are
match: a string that matched the pattern (see <amatch>)
buf: the number of the buffer the event was triggered in (see <abuf>)
file: the file name of the buffer the event was triggered in (see <afile>)
data: a table with other relevant data that is passed for some events

For example, this allows you to set buffer-local mappings for some
filetypes:

```vim
vim.api.nvim_create_autocmd("FileType", {
  pattern = "lua",
  callback = function(args)
    vim.keymap.set('n', 'K', vim.lsp.buf.hover, { buffer = args.buf })
  end
})
```

This means that if your callback itself takes an (even optional) argument, you
must wrap it in function() end to avoid an error:

```vim
vim.api.nvim_create_autocmd('TextYankPost', {
  callback = function() vim.highlight.on_yank() end
})
```
