---
id: Dynamic terminal padding
aliases:
  - Dynamic terminal padding
tags: []
---

# Dynamic terminal padding


   -  Neovim would look bad if terminal has padding as top is covered by bufferline and nvimtree etc with left.
   -  So we can dynamically set padding if nvim's opened or not. So add padding on VimLeavePre and remove padding on VimEnter events.
   -  Below is an example for kitty terminal, you can apply the same logic for other terminals!

```lua
local autocmd = vim.api.nvim_create_autocmd

autocmd("VimEnter", {
  command = ":silent !kitty @ set-spacing padding=0 margin=0",
})

autocmd("VimLeavePre", {
  command = ":silent !kitty @ set-spacing padding=20 margin=10",
})
```

## Restore cursor position

```lua
local autocmd = vim.api.nvim_create_autocmd

autocmd("BufReadPost", {
  pattern = "*",
  callback = function()
    local line = vim.fn.line "'\""
    if
      line > 1
      and line <= vim.fn.line "$"
      and vim.bo.filetype ~= "commit"
      and vim.fn.index({ "xxd", "gitrebase" }, vim.bo.filetype) == -1
    then
      vim.cmd 'normal! g`"'
    end
  end,
})
```
### GotoTab

```lua
local tabline = require("tabline")
tabline.setup()

```lua

