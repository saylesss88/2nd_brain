---
id: NULL_LSS
aliases: []
tags: []
---

```lua
{ -- Null LS {{{
    "nvimtools/none-ls.nvim",
    event = { "BufReadPre", "BufNewFile" },
    dependencies = {
      "williamboman/mason.nvim",
      "nvim-lua/plenary.nvim",
    },
    opts = function()
      local null_ls = require("null-ls")
      return {
        debug = false,
        sources = {
          -- Formatters run in the shown order.

          null_ls.builtins.diagnostics.actionlint,
          null_ls.builtins.diagnostics.buf,
          null_ls.builtins.diagnostics.flake8,
          null_ls.builtins.diagnostics.golangci_lint,
          null_ls.builtins.diagnostics.hadolint,
          null_ls.builtins.diagnostics.selene,

          null_ls.builtins.formatting.autopep8,
          null_ls.builtins.formatting.buf,
          null_ls.builtins.formatting.cbfmt.with({
            extra_args = { "--config", vim.fn.stdpath("config") .. "/scripts/cbfmt.toml" },
          }),
          null_ls.builtins.formatting.fixjson,
          null_ls.builtins.formatting.gofumpt,
          null_ls.builtins.formatting.prettier.with({
            disabled_filetypes = { "html" },
            extra_args = function(params)
              return params.options
                and params.options.tabSize
                and { "--tab-width", params.options.tabSize }
            end,
          }),
          null_ls.builtins.formatting.rustfmt.with({ extra_args = { "--edition=2021" } }),
          null_ls.builtins.formatting.shfmt,
          null_ls.builtins.formatting.stylua.with({
            extra_args = { "--indent-type=Spaces", "--indent-width=2", "--column-width=100" },
          }),
          null_ls.builtins.formatting.uncrustify.with({
            extra_args = function()
              return { "-c", vim.fn.findfile("uncrustify.cfg", ".;"), "--no-backup" }
            end,
          }),
        },
        root_dir = require("null-ls.utils").root_pattern(
          ".null-ls-root",
          ".neoconf.json",
          "Makefile",
          ".git"
        ),
      }
    end,
```
