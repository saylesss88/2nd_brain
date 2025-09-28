---
title: Table Indices
date: 2024-09-27
tags: [lua, tables]
---

# Table Indices

Each table can store values with different types of indices, and it grows as
needed to accommodate new entries:

```lua
> a = {}    -- empty table
> -- create 1000 new entries
> for i = 1, 1000 do a[i] = i*2 end
> a[9]      --> 18
> a["x"] = 10
> a["x"]    --> 10
> a["y"]    --> nil
```

- Note the last line: like global variables, table fields evaluate to nil when
  not initialized. Also like global variables, we can assign nil to a table field
  to delete it.

Lua supports this representation by providing
a.name as syntactic sugar for a["name"]. Therefore, we could write the last lines
of the previous example in a cleaner manner as follows:

```Lua
> a = {}        -- empty table
> a.x = 10      -- same as a["x"] = 10
> a.x           --> 10    -- same as a["x"]
> a.y           --> nil    -- same as a["y"]
```

- For Lua, the two forms are equivalent and can be intermixed freely.

When in doubt about the actual types of your indices, use an explicit
conversion to be sure:

```Lua
> i = 10; j = "10"; k = "+10"
> a = {}
> a[i] = "number key"
> a[j] = "string key"
> a[k] = "another string key"
> a[i]                     --> number key
> a[j]                     --> string key
> a[k]                     --> another string key
> a[tonumber(j)]           --> number key
> a[tonumber(k)]           --> number key
```
