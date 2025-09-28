---
title: Metatables and Metamethods
date: 2024-09-27
tags: [lua, tables]
---

# Metatables and Metamethods

- A table can have a metatable that gives the table operator-overloadish
  behavior.

```lua
f1 = {a = 1, b = 2}  -- Represents the fraction a/b
f2 = {a = 2, b = 3}
```

-- This would fail:
`s = f1 + f2`

```lua
metafraction = {}

function metafraction.__add(f1, f2)
  sum = {}
  sum.b = f1.b * f2.b
  sum.a = f1.a * f2.b + f2.a * f1.b
  return sum
end

setmetatable(f1, metafraction)
setmetatable(f2, metafraction)

s = f1 + f2  -- call _add(f1, f2) on f1's metatable
```

- f1, f2 have no key for their metatable, unlike
-
- prototypes in js, so you must retrieve it as in

- getmetatable(f1). The metatable is a normal table

- with keys that Lua knows about, like \_\_add.

- But the next line fails since s has no metatable:

- `t = s + s`

- Class-like patterns given below would fix this.

## An \_\_index on a metatable overloads dot lookups:

```Lua
defaultFavs = {animal = 'gru', food = 'donuts'}
myFavs = {food = 'pizza'}
setmetatable(myFavs, {__index = defaultFavs})
eatenBy = myFavs.animal  -- works! thanks, metatable
```

- Direct table lookups that fail will retry using the metatable's \_\_index value,
  and this recurses.

- An \_\_index value can also be a function(tbl,, key) for more customized
  lookups.

- Values of \_\_index, add, .. are called metamethods.

- Ful list. Here is a table with the metamethod.

```Lua
__add(a, b)                     for a + b
__sub(a, b)                     for a - b
__mul(a, b)                     for a * b
__div(a, b)                     for a / b
__mod(a, b)                     for a % b
__pow(a, b)                     for a ^ b
__unm(a)                        for -a
__concat(a, b)                  for a .. b
__len(a)                        for #a
__eq(a, b)                      for a == b
__lt(a, b)                      for a < b
__le(a, b)                      for a <= b
__index(a, b)  <fn or a table>  for a.b
__newindex(a, b, c)             for a.b = c
__call(a, ...)                  for a(...)
```
