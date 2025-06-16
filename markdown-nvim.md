---
id: markdown-nvim
aliases:
  - markdown.nvim
tags: []
---

# markdown.nvim

## Configuration

Detailed plugin configuration information can be found in the help doc
(:h markdown.configuration).

A call to require("markdown").setup() is necessary for commands and keybindings
to be registered in markdown buffers.

- [ ]A table of configuration options can optionally be passed to the setup() function.
  Any fields in the table will overwrite the corresponding default. markdown.nvim uses the following defaults:
  Testing **_bold and italic_** from ~~strikethrough~~

```lua
{
  -- Disable all keymaps by setting mappings field to 'false'.
  -- Selectively disable keymaps by setting corresponding field to 'false'.
  mappings = {
    inline_surround_toggle = "gs", -- (string|boolean) toggle inline style
    inline_surround_toggle_line = "gss", -- (string|boolean) line-wise toggle inline style
    inline_surround_delete = "ds", -- (string|boolean) delete emphasis surrounding cursor
    inline_surround_change = "cs", -- (string|boolean) change emphasis surrounding cursor
    link_add = "gl", -- (string|boolean) add link
    link_follow = "gx", -- (string|boolean) follow link
    go_curr_heading = "]c", -- (string|boolean) set cursor to current section heading
    go_parent_heading = "]p", -- (string|boolean) set cursor to parent section heading
    go_next_heading = "]]", -- (string|boolean) set cursor to next section heading
    go_prev_heading = "[[", -- (string|boolean) set cursor to previous section heading
  },
  inline_surround = {
    -- For the emphasis, strong, strikethrough, and code fields:
    -- * 'key': used to specify an inline style in toggle, delete, and change operations
    -- * 'txt': text inserted when toggling or changing to the corresponding inline style
    emphasis = {
      key = "i",
      txt = "*",
    },
    strong = {
      key = "b",
      txt = "**",
    },
    strikethrough = {
      key = "s",
      txt = "~~",
    },
    code = {
      key = "c",
      txt = "`",
    },
  },
  link = {
    paste = {
      enable = true, -- whether to convert URLs to links on paste
    },
  },
  toc = {
    -- Comment text to flag headings/sections for omission in table of contents.
    omit_heading = "toc omit heading",
    omit_section = "toc omit section",
    -- Cycling list markers to use in table of contents.
    -- Use '.' and ')' for ordered lists.
    markers = { "-" },
  },
  -- Hook functions allow for overriding or extending default behavior.
  -- Called with a table of options and a fallback function with default behavior.
  -- Signature: fun(opts: table, fallback: fun())
  hooks = {
    -- Called when following links. Provided the following options:
    -- * 'dest' (string): the link destination
    -- * 'use_default_app' (boolean|nil): whether to open the destination with default application
    --   (refer to documentation on <Plug> mappings for explanation of when this option is used)
    follow_link = nil, },
  on_attach = nil, -- (fun(bufnr: integer)) callback when plugin attaches to a buffer
}
```

`on_attach` is useful for creating additional buffer-only keymaps:

```lua
require("markdown").setup({
  on_attach = function(bufnr)
                local function toggle(key)
                    return "<Esc>gv<Cmd>lua require'markdown.inline'"
                        .. ".toggle_emphasis_visual('" .. key .. "')<CR>"
                end

                vim.keymap.set("x", "<C-b>", toggle("b"), { buffer = bufnr })
                vim.keymap.set("x", "<C-i>", toggle("i"), { buffer = bufnr })
                vim.keymap.set("x", "<C-c>", toggle("c"), { buffer = bufnr })
                vim.keymap.set("x", "<C-t>", toggle("s"), { buffer = bufnr })
            end
})
```

`markdown-nvim` can be configured to ~~support~~ standard/typical inline style key-
bindings in visual mode like `<C-b>` for strong/bold and `<C-i>` for italics

- [x] Test
- [-]
