---
id: null_ls
aliases: []
tags: []
---

```lua
local null_ls = pRequire("null-ls")

if not null_ls then
  return
end

null_ls.setup({
  debug = false,
  sources = require("insis.env").getNulllsSources(),
  -- #{m}: message
  -- #{s}: source name (defaults to null-ls if not specified)
  -- #{c}: code (if available)
  diagnostics_format = "[#{s}] #{m}",
  on_attach = function(_)
    -- vim.cmd([[ command! Format execute 'lua vim.lsp.buf.formatting()']])
    -- if client.resolved_capabilities.document_formatting then
    --   vim.cmd("autocmd BufWritePre <buffer> lua vim.lsp.buf.formatting_sync()")
    -- end
  end,
})
```

```lua markdown.lua
--- @param config MarkdownConfig
return function(config)
  return {
    getFormatOnSavePattern = function()
      if config.format_on_save then
        return { "*.md", "*.mdx" }
      end
      return {}
    end,

    getTSEnsureList = function()
      return { "markdown", "markdown_inline" }
    end,

    getLSPEnsureList = function()
      return {} -- empty array
    end,

    getLSPConfigMap = function()
      return {}
    end,

    getToolEnsureList = function()
      if config.formatter == "prettier" then
        return { "prettier" }
      end
      return {}
    end,

    getNulllsSources = function()
      local null_ls = pRequire("null-ls")
      if not null_ls then
        return {}
      end
      if config.formatter == "prettier" then
        return { null_ls.builtins.formatting.prettier.with({
          filetypes = { "markdown" },
        }) }
      end
      return {}
    end,

    getNeotestAdapters = function()
      return {}
    end,
  }
end
```
