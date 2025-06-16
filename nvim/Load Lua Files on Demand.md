---
id: Load Lua Files on Demand
aliases:
  - Load Lua Files on Demand
tags: []
---

# Load Lua Files on Demand

If you want to load Lua files on demand, you can place them in the `lua/` directory
in your `runtimepath` and load them with `require`. (This is the Lua equivalent of 
Vimscript's `autoload` mechanism).

Let's assume you have the following directory structure:

```shell
~/.config/nvim
|-- after/
|-- ftplugin/
|-- lua/
|   |-- myluamodule.lua
|   |-- other_modules/
|       |-- anothermodule.lua
|       |-- init.lua
|-- plugin/
|-- syntax/
|-- init.vim
```
- Then the following Lua code will load `myluamodule.lua`:

```lua
require('myluamodule')
```
> Note the absence of a `.lua` extension.

Similarly, loading `other_modules/anothermodule.lua` is done via:

```lua
require('other_modules/anothermodule')
-- or
require('other_modules.anothermodule')
```
> Note how "submodules" are just subdirectories; the `.` is equivalent to the path
separator `/` (even on Windows)

A folder containg an `init.lua`file can be required directly, without having to
specify the name of the file:

```lua
require('other_modules')  -- loads `other_modules/init.lua`
```
Requiring a nonexistent module or a module which contains syntax errors aborts
the currently executing script. pcall() may be used to catch such errors. The
following example tries to load the module_with_error and only calls one of
its functions if this succeeds and prints an error message otherwise:

```lua
local ok, mymod = pcall(require, 'module_with_error')
if not ok then
  print("Module had an error")
else
  mymod.function()
end
```
In contrast to :source, require() not only searches through all lua/ directories
under 'runtimepath', it also caches the module on first use. Calling
require() a second time will therefore _not_ execute the script again and
instead return the cached file. To rerun the file, you need to remove it from
the cache manually first:

```lua
package.loaded['myluamodule'] = nil
require('myluamodule')    -- read and execute the module again from disk
```
