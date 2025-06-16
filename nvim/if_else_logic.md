---
id: if_else_logic
aliases:
  - if_else_logic
tags: []
---

# if_else_logic

To integrate LazyVim Plugins into your own config change:

```lua
config = function(_, opts)
    require("refactoring").setup(opts)
    if LazyVim.has("telescope.nvim") then
      LazyVim.on_load("telescope.nvim", function()
        require("telescope").load_extension("refactoring")
      end)
    end
  end,
```

To this:

```lua
config = function(_, opts)
    require("refactoring").setup(opts)
    local has_telescope = pcall(require, "telescope.nvim")
    if has_telescope then
        require("telescope").load_extension("refactoring")
    end
end
```

- We replaced `LazyVim.has("telescope.nvim")` with `pcall(require, "telescope.nvim)`
  This checks if the `telescope.nvim` module can be loaded without relying on
  LazyVim.
- If `telescope.nvim` is available, it loads the refactoring extension.

- The `config` function is often used for setting up plugins or customizing
behavior within Neovim.
- The `opts` parameter allows you to pass specific options or settings to the
plugin. These options are usually defined by the plugin itself.
- You can think of `opts` as a way to fine-tune how the plugin behaves within
your nvim environment.
