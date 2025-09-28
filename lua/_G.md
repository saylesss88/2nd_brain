---
title: _G special table of all globals
date: 2024-09-27
tags: [lua, tag2]
---

# \_G special table of all globals

```lua
print(_G['_G'] == _G)
```

**Breakdown:**

- `_G`: This is a special global variable in Lua that represents the global
  environment table. It contains all global variables and functions.

- `_G['_G']`: This expression is accessing the value stored in the `_G` table.

- `==`: Equality operator.

**Explanation:**

The code essentially checks if the value stored in the `_G` table under the key
`_G` is the same as the `_G` table itself. In Lua, the global environment table is
self-referential; it's stored within itself.

Therefore, the output of the code will be true.

This demonstrates that the `_G` variable is indeed a reference to the global
environment table, and accessing it within itself using the key `_G` returns the
same table.
