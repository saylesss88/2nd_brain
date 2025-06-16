---
id: NvimFunction
aliases:
  - NvimFunction
tags: []
---

# NvimFunction

```lua
local telescope_loaded = false -- Flag to track Telescope loading status

if nvim.has("telescope.nvim") then
  -- Telescope is already loaded, require directly
  telescope_loaded = true
  require("telescope")

else
  -- Telescope not loaded yet, use nvim.on_load or packer.nvim's lazy loading (if applicable)
  nvim.on_load("telescope.nvim", function()
    telescope_loaded = true
    require("telescope")
  end)

  -- If using packer.nvim, consider using its lazy loading functionality:
  -- use { "nvim-telescope/telescope.nvim" }
end

-- Your Telescope configuration goes here
-- (assuming telescope_loaded is set to true)

if telescope_loaded then
  -- Telescope configuration that requires Telescope to be loaded
  telescope.setup {
    -- Your Telescope options here
  }
end
```

Explanation:

    telescope_loaded Flag: A flag is introduced to track whether Telescope has been loaded successfully.
    Check for Telescope: The if nvim.has(...) check remains to verify if Telescope is already available.
    Direct Require (if Loaded): If Telescope is loaded, we set telescope_loaded to true and require it directly using require("telescope").
    nvim.on_load Fallback: If Telescope is not yet loaded, we use nvim.on_load("telescope.nvim", ...) to ensure the require happens asynchronously after Telescope is fully loaded. This avoids potential errors.
    Packer.nvim Lazy Loading (Optional): If you're using packer.nvim, include the telescope.nvim plugin in your use block to leverage its lazy loading mechanism (recommended).
    Telescope Configuration:
        The actual Telescope configuration code is placed after the checks, ensuring it's only executed when Telescope is loaded.
        We check the telescope_loaded flag before calling telescope.setup.
    Optional Telescope-Specific Configuration:
        If you have configuration that specifically requires Telescope to be loaded (e.g., keybindings), wrap that code inside the if telescope_loaded then block.

Benefits of This Approach:

    Robust Error Handling: Ensures Telescope is loaded before using it, preventing potential errors.
    Flexibility: Caters to both direct require (if loaded) and lazy loading (using nvim.on_load or packer.nvim).
    Clarity: Uses a flag for easier tracking of Telescope's loading status.

Additional Considerations:

    Adjust the key binding configuration based on your specific setup.
    Refer to the Telescope documentation for detailed configuration options: https://github.com/nvim-telescope/telescope.nvim
