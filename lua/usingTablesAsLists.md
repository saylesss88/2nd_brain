---
title: Using Tables as Lists
date: 2024-09-27
tags: [lua, tables]
---

# Using Tables as Lists

```lua
-- List literals implicitly set up int keys:
v = {'value1', 'value2', 1.21, 'gigawatts'}
for i = 1, #v do  -- #v is the size of v for lists.
  print(v[i])  -- Indices start at 1 !! SO CRAZY!
end
-- A 'list' is not a real type. v is just a table
-- with consecutive integer keys, treated as a list.
```

Output:

```lua
value1
value2
1.21
gigawatts
```

[[metaTablesAndMetamethods.md]]
