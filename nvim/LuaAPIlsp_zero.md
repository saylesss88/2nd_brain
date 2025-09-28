---
id: LuaAPIlsp_zero
aliases:
  - Lua API
tags: []
---

# Lua API

**default_keymaps(opts)**

Create the keybindings bound to built-in LSP functions.

The `opts` table supports the following properties:

  - buffer: (optional) Number. The "id" of an open buffer. If the number 0 is
  provided then the keymaps will be effective in the current buffer.

  - preserve_mappings: (optional) Boolean. Default is true, when set to `true`
  lsp-zero will not override your existing keybindings.

  - exclude: (optional) Table. A list of string, must be valid keybindings.
  lsp-zero will preserve the behavior of these keybindings.

```Lua
local lsp_zero = require("lsp-zero")

lsp_zero.on_attach(function(client, bufnr)
  lsp_zero.default_keymaps({ buffer = bufnr })
end)
```

## LSP Actions

  - `K`: Displays hover information for the symbol under the cursor in a
  floating window.

  - `gd`: Jumps to the definition of the symbol under the cursor.

  - `gi`: List the implementations of the symbol under the cursor in the
  quickfix window.

  - `go`: Jumps to the definition of the type of the symbol under the cursor.

  - `gr`: Lists all the references to the symbol under the cursor in the

  - `gs`: Displays signature information

  - `<F2>`: Renames all references to the symbol under the cursor.

  - `<F3>`: Format code in the current buffer.

  - `<F4>`: Selects a code action available at the current cursor position.

  - `gl`: Displays diagnostics in a floating window.

  - `[d`: Moves to the next diagnostic in the current buffer.

  - `]d`: Moves to the previous diagnostic in the current buffer.

### on_attach(callback)

Executes the `{callback}` function every time a language server is attached to a
buffer.

This is where you can declare your own keymaps and commands.

```lua
local lsp_zero = require('lsp-zero')

lsp_zero.on_attach(function(client, bufnr)
  lsp_zero.default_keymaps({buffer = bufnr})

  local opts = {buffer = bufnr}
  vim.keymap.set('n', '<leader>r', '<cmd>lua vim.lsp.buf.rename()<cr>', opts)
  -- more code  ...
end)
```

#### default_setup(server)

Configures `{server}` with the default config provided by lspconfig.

This is meant to be used with mason-lspconfig.nvim, in order to help configure
automatic setup of language servers. It can be added as a default handler in
the setup function of the module mason-lspconfig.

```lua
local lsp_zero = require('lsp-zero')

lsp_zero.on_attach(function(client, bufnr)
  lsp_zero.default_keymaps({buffer = bufnr})
end)

require('mason').setup({})
require('mason-lspconfig').setup({
  handlers = {
    lsp_zero.default_setup,
  },
})
```
