---
id: lsp_functions
aliases: []
tags: []
---


This function gets run when an LSP attaches to a particular buffer. Every time a
new file is opened that is associated with an lsp, this function gets executed
to configure the current buffer.

```lua
vim.api.nvim_create_autocmd({ "LspAttach" }, {
  group = vim.api.nvim_create_augroup("LspFuncs", { clear = true }),
  callback = function(event)
})
```

This is a helper function to let us more easily define mappings specific for LSP
related items. It sets the mode, buffer, and description for us each time:

```lua
local map = function(keys, func, desc)
  vim.keymap.set('n', keys, func, { buffer = event.buf, desc = 'LSP: ' .. desc })
end
```

The following two autocommands are used to highlight references of the word
under the cursor when your cursor rests there for a little while.

-- When you move your cursor, the highlights will be cleared (the second
autocommand)

```lua
 vim.api.nvim_create_autocmd({ 'CursorMoved', 'CursorMovedI' }, {
              buffer = event.buf,
              group = highlight_augroup,
              callback = vim.lsp.buf.clear_references,
            })

            vim.api.nvim_create_autocmd('LspDetach', {
              group = vim.api.nvim_create_augroup('kickstart-lsp-detach', { clear = true }),
              callback = function(event2)
                vim.lsp.buf.clear_references()
                vim.api.nvim_clear_autocmds { group = 'kickstart-lsp-highlight', buffer = event2.buf }
              end,
            })
          end
```
-- LSP servers and clients are able to communicate to each other what features they support.
      --  By default, Neovim doesn't support everything that is in the LSP specification.
      --  When you add nvim-cmp, luasnip, etc. Neovim now has *more* capabilities.
      --  So, we create new capabilities with nvim cmp, and then broadcast that to the servers.

```lua
local capabilities = vim.lsp.protocol.make_client_capabilities()
capabilities = vim.tbl_deep_extend('force', capabilities, require('cmp_nvim_lsp').default_capabilities())


```

