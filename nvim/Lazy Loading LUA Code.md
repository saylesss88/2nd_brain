---
id: Lazy Loading LUA Code
aliases:
  - Lazy Loading LUA Code
tags: []
---

# Lazy Loading LUA Code

You may not always want to run code whenever Neovim starts or when a FileType
event triggers. Instead you may want to run logic on-demand, either by calling a
function directly or via keymap.

The simplest way to do that, is to place a single file in a lua/ folder. This
folder can be in any of the runtimepath or packages locations. For example, in
lua/foobar.lua you would write:

```lua
-- Using `M` is a common Lua convention, `M` stand for module
-- It's used for a table that contains all exported functions and properties
-- (Exported because it's returned at the end of the file)
local M = {}

function M.do_something()
  print('Hello world')
end

return M

```

A user would then load it on-demand using `:lua require('foobar').do_something()`

`require` is a Lua function to search a module, load and execute it.

- The first `require` call caches the module.
- This means a second `require('foobar')` call won't run all the logic.

Consider the following example:

```lua
local M = {}

function M.do_something()
  print('This is printed whenever you call do_something()')
end

print('This is printed only once')

return M

```
