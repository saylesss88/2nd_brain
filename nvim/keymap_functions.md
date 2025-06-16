---
id: keymap_functions
aliases: []
tags: []
---

Mappings can be created using vim.keymap.set(). This function takes three
mandatory arguments:
{mode} is a string or a table of strings containing the mode prefix for which
the mapping will take effect. The prefixes are the ones listed in :map-modes,
or "!" for :map!, or empty string for :map.
{lhs} is a string with the key sequences that should trigger the mapping.
{rhs} is either a string with a Vim command or a Lua function that should be
executed when the {lhs} is entered. An empty string is equivalent to <Nop>,
which disables a key.

Examples:

```vim
-- Normal mode mapping for Vim command
vim.keymap.set('n', '<Leader>ex1', '<cmd>echo "Example 1"<cr>')
-- Normal and Command-line mode mapping for Vim command
vim.keymap.set({'n', 'c'}, '<Leader>ex2', '<cmd>echo "Example 2"<cr>')
-- Normal mode mapping for Lua function
vim.keymap.set('n', '<Leader>ex3', vim.treesitter.start)
-- Normal mode mapping for Lua function with arguments
vim.keymap.set('n', '<Leader>ex4', function() print('Example 4') end)
```

You can map functions from Lua modules via

```vim
vim.keymap.set('n', '<Leader>pl1', require('plugin').action)
```

Note that this loads the plugin at the time the mapping is defined. If you
want to defer the loading to the time when the mapping is executed (as for
autoload functions), wrap it in function() end:

```vim
vim.keymap.set('n', '<Leader>pl2', function() require('plugin').action() end)
```

The fourth, optional, argument is a table with keys that modify the behavior
of the mapping such as those from :map-arguments. The following are the most
useful options:

- buffer: If given, only set the mapping for the buffer with the specified number;
  0 or true means the current buffer.

```vim
-- set mapping for the current buffer
vim.keymap.set('n', '<Leader>pl1', require('plugin').action, { buffer = true })
-- set mapping for the buffer number 4
vim.keymap.set('n', '<Leader>pl1', require('plugin').action, { buffer = 4 })
```

- silent: if set to `true`, suppress output such as error messages

```vim
vim.keymap.set('n', '<Leader>pl1', require('plugin').action, { silent = true })
```

expr: If set to true, do not execute the {rhs} but use the return value as input.
Special keycodes are converted automatically. For example, the following mapping
replaces <down> with <c-n> in the popupmenu only:

```vim
vim.keymap.set('c', '<down>', function()
  if vim.fn.pumvisible() == 1 then return '<c-n>' end
  return '<down>'
end, { expr = true })
```
