---
id: 1721307721-clean-lspconfig-syntax
aliases:
  - Clean Lspconfig syntax
tags: []
---

# Clean Lspconfig syntax

- If you have multiple servers configured in your lspconfig, then you can just
put them in a table and loop through them for cleaner syntax.

```lua
local configs = require "nvchad.configs.lspconfig"

local servers = {
  html = {},
  awk_ls = {},
  bashls = {},

  pyright = {
    settings = {
      python = {
        analysis = {
          autoSearchPaths = true,
          typeCheckingMode = "basic",
        },
      },
    },
  },
}

for name, opts in pairs(servers) do
  opts.on_init = configs.on_init
  opts.on_attach = configs.on_attach
  opts.capabilities = configs.capabilities

  require("lspconfig")[name].setup(opts)
end
```

[[Dynamic terminal padding.md]]
