---
id: Yanky
aliases: []
tags: []
---

## Core Functionality:

Yanking:
`y`: Yanks the selected text into the Yanky ring.

`p`: Puts the last yanked text after the cursor.

`P`: Puts the last yanked text before the cursor.

`gp`: Puts the last yanked text after the current selection.

`gP`: Puts the last yanked text before the current selection.

Ring Navigation:

`[y`: Cycles forward through the yank history.

`]y`: Cycles backward through the yank history.

Indentation:

`]p`: Puts the last yanked text after the cursor, indented to match the current line.

`[p`: Puts the last yanked text before the cursor, indented to match the current line.

`>p`: Puts the last yanked text after the cursor and indents it right.

`<p`: Puts the last yanked text after the cursor and indents it left.

`>P`: Puts the last yanked text before the cursor and indents it right.

`<P`: Puts the last yanked text before the cursor and indents it left.

Filtering:
`[y`: Cycles forward through the yank history.
`gP`: Puts the last yanked text before the current selection.

`=p`: Puts the last yanked text after the cursor, applying a filter.

`=P`: Puts the last yanked text before the cursor, applying a filter.

Configuration Breakdown:

local map = vim.keymap.set: This line defines a local variable map for setting key mappings.

local yanky = { ... }: This creates a table yanky to store the Yanky plugin configuration.

dependencies = { ... }: This specifies the dependencies required by the Yanky plugin, including the SQLite library for non-Windows systems.

opts = function() ... end: This defines a function that returns the Yanky plugin options.

mapping = require "yanky.telescope.mapping": Loads the Yanky Telescope mappings.

mappings = mapping.get_defaults(): Gets the default Yanky Telescope mappings.

mappings.i["<c-p>"] = nil: Removes the default Ctrl+p mapping in insert mode.

    highlight = { timer = 200 }: Sets the highlight timer for yanked text to 200 milliseconds.

    ring = { storage = jit.os:find "Windows" and "shada" or "sqlite" }: Configures the Yanky ring storage based on the operating system.

    picker = { telescope = { ... } }: Sets the Telescope picker options.

keys = { ... }: Defines the key mappings for Yanky commands.



Usage:


Install the Yanky plugin: Use your preferred Neovim plugin manager to install gbprod/yanky.nvim.


Configure the plugin: Place the provided configuration code in your Neovim configuration file (e.g., init.vim).

Use the commands: Trigger Yanky commands using the specified key bindings (e.g., y, p, P, etc.).
