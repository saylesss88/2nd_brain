```lua
  -- Example for neo-tree.nvim
    {
      "nvim-neo-tree/neo-tree.nvim",
        keys = {
          { "<leader>ft", "<cmd>Neotree toggle<cr>", desc = "NeoTree" },
        },
        config = function()
          require("neo-tree").setup()
        end,
    }
```
