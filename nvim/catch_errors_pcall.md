---
id: catch_errors_pcall
aliases:
  - catch_errors_pcall
tags: []
---

# catch_errors_pcall

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

In contrast to `:source`, `require()` not only searches through all `lua/` directories
under `runtimepath`, it also caches the module on first use. Calling `require`
a second time will therefore _not_ execute the script again and instead return
the cached file.

To rerun the file, you need to remove it from the cache manually first:

```lua
package.loaded['myluamodule'] = nil
require('myluamodule')    -- read and execute the module again from disk
```
