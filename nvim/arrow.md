---
id: arrow
aliases: []
tags: []
---

Basic Usage

    Leader Key: The leader_key is set to ;. This means you can press ; followed by the appropriate command to interact with arrow.nvim.
    Buffer Leader Key: The buffer_leader_key is set to m. This is used for per-buffer mappings.

Commands

    Bookmark a File: Press ; followed by a to add a bookmark.
    Open Bookmarks Menu: Press ; to open the bookmarks menu.
    Navigate Bookmarks: Use the arrow keys or j/k to navigate through the bookmarks.
    Open a Bookmark: Press Enter to open the selected bookmark.
    Delete a Bookmark: Press d to enter delete mode, then select the bookmark to delete.

Advanced Configuration

You can further customize arrow.nvim with additional options. Here’s an example:

```lua
require('arrow').setup({
  show_icons = true,
  leader_key = ';',
  buffer_leader_key = 'm',
  always_show_path = false,
  separate_by_branch = false,
  save_path = function()
    return vim.fn.stdpath("cache") .. "/arrow"
  end,
  mappings = {
    edit = "e",
    delete_mode = "d",
    clear_all_items = "C",
    toggle = "s",
    open_vertical = "v",
    open_horizontal = "-",
    quit = "q",
    remove = "x",
    next_item = "]",
    prev_item = "[",
  },
  window = {
    width = "auto",
    height = "auto",
    row = "auto",
    col = "auto",
    border = "double",
  },
  per_buffer_config = {
    lines = 4,
    sort_automatically = true,
  },
})
```
