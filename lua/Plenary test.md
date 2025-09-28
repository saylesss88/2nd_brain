---
id: Plenary test
aliases:
  - Plenary test
tags: []
---

# Plenary test

This test demonstrates a `describe` block that contains two tests defined with `it`
blocks, the describe block also contains a `before_each` call that gets called
before each test.

```lua
describe("some basics", function()
  local bello = function(boo)
    return "bello " .. boo
  end

  local bounter

  before_each(function()
    bounter = 0
  end)

  it("some test", function()
    bounter = 100
    assert.equals("bello Brian", bello("Brian"))
  end)

  it("some other test", function()
    assert.equals(0, bounter)
  end)
end)
```

The test `some test` checks that a functions output is as expected based on the
input. The second test `some other test` checks that the variable `bounter` is
reset for each test(as defined in the `before_each` block).

## Running tests

Run the test using `:PlenaryBustedFile <file>`.

```lua
" Run the test in the current buffer
:PlenaryBustedFile %
" Run all tests in the directory "tests/plenary/"
:PlenaryBustedDirectory tests/plenary/
```

Or you can run tests in headless mode to see output in terminal:

```lua
# run all tests in terminal
cd plenary.nvim
nvim --headless -c 'PlenaryBustedDirectory tests'
```

### mocking with luassert

Plenary.nvim comes bundled with luassert a library that's built to extend the
built-int assertions... but it also comes with stubs, mocks and spies!

Sometimes it's useful to test functions that have nvim api function calls within
them, take for example the following example of a simple module that creates a
new buffer and opens in it in a split.

```lua module.lua
local M = {}

function M.realistic_func()
  local buf = vim.api.nvim_create_buf(false, true)
  vim.api.nvim_command("sbuffer " .. buf)
end

return M
```
