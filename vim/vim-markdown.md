---
id: vim-markdown
aliases:
  - vim-markdown
tags: []
---

# vim-markdown

## Requirements

- [!] **markdown** and markdown_inline tree-sitter parsers

### Installation

- lazy.nvim

```lua
{
  "tadmccorkle/markdown.nvim",
  ft = "markdown", -- or 'event = "VeryLazy"'
  opts = {
    -- configuration here or empty for defaults
  },
}
```

#### Getting Help

```vim
:help markdown.nvim
```

##### Configuration

- Detailed **plugin** info can be found in the help doc `:help markdown.nvim`

A call to `require('markdown').setup()` is necessary for commands and
keybindings to be registered in markdown buffers.
