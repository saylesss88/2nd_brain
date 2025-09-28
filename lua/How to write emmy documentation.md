---
id: How to write emmy documentation
aliases:
  - How to write emmy documentation
tags: []
---

# How to write emmy documentation

## Brief

Brief is used to describe the documentation of a module.

```md HOWTO.md
---@brief [[
--- This will document a module and will be found at the top of each file. It uses an internal markdown renderer
--- so you don't need to worry about formatting. It will wrap the lines into one paragraph and
--- will make sure that the max line width is < 80.
---
--- To start a new paragraph with a newline.
---
--- To explicitly do a breakline do a `<br>` at the end.<br>
--- This is useful sometimes
---
--- We also support itemize and enumerate
--- - Item 1
---   - Item 1.1 This item will be wrapped as well and the result will be as expected. This is really handy.
---     - Item 1.1.1
---   - Item 1.2
--- - Item 2
---
--- 1. Item
---   1.1. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna
---   aliquyam erat, sed diam voluptua.
---     1.1.1. Item
---   1.2. Item
--- 2. Item
---
--- <pre>
--- You can disable formatting with a
--- pre block.
--- This is useful if you want to draw a table or write some code
--- </pre>
---
---@brief ]]

```
Output:

```md
================================================================================
This will document a module and will be found at the top of each file. It uses
an internal markdown renderer so you don't need to worry about formatting. It
will wrap the lines into one paragraph and will make sure that the max line
width is < 80.

To start a new paragraph with a newline.

To explicitly do a breakline do a `<br>` at the end.
This is useful sometimes

We also support itemize and enumerate
- Item 1
  - Item 1.1 This item will be wrapped as well and the result will be as
    expected. This is really handy.
    - Item 1.1.1
  - Item 1.2
- Item 2

1. Item
  1.1. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy
       eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam
       voluptua.
    1.1.1. Item
  1.2. Item
2. Item

You can disable formatting with a
pre block.
This is useful if you want to draw a table or write some code




```

### Tag

Add a tag to your module. This is suggested:

```md
---@tag your_module
```

Output:

```md
================================================================================
                                                                   *your_module*

```

#### Config

You can configure docgen on file basis. For example you can define how
functions or classes are sorted.

```md
---@config { ['function_order'] = 'ascending', ['class_order'] = 'descending' }

```
