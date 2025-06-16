---
title: lua tables
date: 2024-09-26
tags: [lua, tag2]
---

# Lua tables

Tables are the main (in fact, the only) data structure mechanism in Lua.

- We use tables to represent arrays, sets, records, and many other data
  structures in a simple, uniform, and efficient way.

- When we write `math.sin`, we think about "the function `sin` from the `math`
  library".

  - For Lua, this expression means "index the table math using the string" `sin`
    as the key.

- Tables in Lua are neither values nor variables; they are objects.

  - You might think of a table as a dynamically-allocated object; programs
    manipulate only references (or pointers) to them.

## Creating tables

We create tables by means of a _constructor expression_, which in simplest form
is written as `{}`.

```lua
> a = {}                  -- create a table and assign its reference
> k = "x"
> a[k] = 10               -- new entry, with key="x" and value=10
> a[20] = "great"         -- new entry, with key=20 and value="great"
> a["x"]                  --> 10
> k = 20
> a[k]                    --> "great"
> a["x"] = a["x"] + 1     -- increments entry "x"
> a["x"]                  --> 11
```

- A table is always anonymous. There is no fixed relationship between a variable
  that holds a table and the table itself:

```lua
> a = {}
> a["x"] = 10
> b = a              -- 'b' refers to the same table as 'a'
> b["x"]             --> 10
> b["x"] = 20
> a["x"]             --> 20
> a = nil            -- only 'b' still refers to the table
> b = nil            -- no references left to the table
```
