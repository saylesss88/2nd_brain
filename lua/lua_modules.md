---
id: lua_modules
aliases:
  - lua_modules
tags: []
---

# lua_modules

Import code from other files

```lua
require("path")

-- for example in ~/.config/nvim/lua , all dirs and files are accessable via require
-- Do note that all files in that lua folder are in path!
-- ~/.config/nvim/lua/abc.lua 
-- ~/.config/nvim/lua/abc/init.lua

 require "abc"

-- both do the same thing

```
## vim.tbl_deep_extend

This is a neovim function which is used for merging tables and their values 
recursively. Most plugins use it for merging config tables.

```lua
-- table 1
local person = {
    name = "Tom",
    age = 37,
    skills = {"python", "lua"},
}

-- table 2
local someone = {
    name = "siduck",
    skills = {"js", "go"},
}

-- "force" will overwrite equal values from the someone table over the peson tbl
local resule = vim.tbl_deep_extend("force", person, someone)

-- result :
    {
    name = "siduck",
    age = 37,
    skills = {"js", "go"},
}
-- The list tables wont merge cuz they don't have keys
```
