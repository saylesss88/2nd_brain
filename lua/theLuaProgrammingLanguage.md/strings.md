---
id: strings
aliases:
  - strings
tags:
  - lua
  - strings
date: "2024-09-27"
title: Strings
---

# strings

Strings in Lua are sequences of bytes. The Lua core is agnostic about how these
bytes encode text.

- Strings in Lua are immutable values.

- The standard string library that comes with Lua assumes one-byte characters,
  but it can handle UTF-8 strings quite reasonably.

```lua
a = "one string"
b = string.gsub(a, "one", "another")  -- change string parts
print(a)                              --> one string
print(b)                              --> another string
 ```

 - Strings are subject to automatic memory management. This means that we don't
   have to worry about allocation and deallocation of strings; Lua handles it
   for us.

We can get the length of a string using the length operator (denoted by #):

```lua
a = "hello"
print(#a)            --> 5
print(#"good bye")   --> 8
```

- The operator always counts the length in bytes, which is not the same as
  characters in some encodings.

We can concatenate two strings with the concat operator `..` If any operand is a
number, Lua converts this number to a string:

```lua
> "Hello " .. "World"      --> Hello World
> "result is " .. 3        --> result is 3
```

Remember that strings in Lua are immutable values. The concatenation operator
always creates a new string, without any modification to its operands:

```lua
> a = "Hello"
> a .. " World"     --> Hello World
> a                 --> Hello
```

[[literalStrings.md]]
