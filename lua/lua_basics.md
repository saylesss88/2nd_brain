---
id: lua_basics
aliases:
  - Lua Basics
tags: []
---

# Lua Basics

## Comments

```lua
-- comment

--[[
  multiline
  comment
]]
```

### Variables

```lua
local x = 10 -- number
local name = "sid" -- string
local isAlive = true -- boolean
local a = nil --no value or invalid value

-- increment in numbers
local n = 1
n = n + 1
print(n) -- 2

-- strings
-- Concatenate strings
local phrase = "I am"
local name = "Sid"

print(phrase .. " " .. name) -- I am Sid
print("I am " .. "Sid")
```

> :Note: In Lua, the concatenation operator `..` is used to concatenate strings.

### Comparison Operators

```lua
== equal
< less than
> greater than
<= less than or equal to
>= greater than or equal to
~= not equal
```

### Conditional Statements

```lua
-- Number Comparison
local age = 10

if age > 18 then
    print("over 18") -- this will not be executed
end

-- elseif and else
age = 20

if age > 18 then
    print("over 18")
elseif age == 18 then
    print("18")
else
    print("kiddo")
end

-- Boolean Comparison
local isAlive = true

if isAlive then
    print("Be grateful")
end

-- String Comparison
local name = "Sid"

if name ~= "Sid" then
    print("Not Sid")
end
```

#### Combining Statements

```lua
local age = 22

if age == 10 and x > 0 then  -- both should be true
    print("kiddo!")
elseif x == 18 or x > 18 then  -- 1 or more are true
    print("over 18")
end
```

#### Functions

```lua
local function print_name(a)
    print(a)
end

-- or

local print_num = function(a)
    print(a)
end

print_num(5)  -- prints 5

-- multiple parameters
function sum(a, b)
    return a + b
end
```

#### Scope

Variables have different scopes. Once the end of the scope is reached, the
values in that scope are no longer accessible.

```lua
function foo()
    local n = 10
end

print(n)  -- nil , n isn't accessible outside foo()
```

### Loops

Different ways to make a loop:

**While**

```lua
local i = 1

while i <= 3 do
    print("hi")
    i = i + 1
end
```

**For**

```lua
for i = 1, 3 do
    print("hi")
end
-- Both print "hi" 3 times
```

**Tables**

- Tables can be used to store complex data.
- Types of tables: arrays (lists) and dicts (key,value)

**Arrays**

- Items within these can be accessed by "index"

```lua
local colors = {"red", "green", "blue"}

print(colors[1]) -- red

-- Different ways to loop through lists
-- #colors is the length of the table, #tablename is the syntax

for i = 1, #colors do  -- #colors is a special expression that returns the len
    print(colors[i])   -- in the `colors` array
end

-- ipairs
for index, value in ipairs(colors) do
    print(colors[index])
    -- or
    print(value)
end

-- If you dont use index or value here then you can replace it with _
for _, value in ipairs(colors) do
    print(value)
end
```

**Dictionaries**

- These contain keys and values:

```lua
local info = {
    name = "sid",
    age = 20,
    isAlive = true
}

-- both print sid
print(info["name"])
print(info.name)

-- Loop by pairs
for key, value in pairs(info) do
    print(key .. " " .. tostring(value))
end

-- prints name sid, age 20 etc
```

**Nested tables**

```lua
-- Nested lists
local data = {
    { "sid", 20 },
    { "tim", 90 },
}

for i = 1, #data do
    print(data[i][1] .. " is " .. data[i][2] .. " years old")
end

-- Nested dictionaries
local data = {
    sid = { aga = 20 },
    tim = { age = 90 },
}
```

[[lua_modules.md]]
