---
title: Frontmatter
date: 2024-09-24
tags: [tag1, tag2]
---

```lua
local ls = require("luasnip")
local s = ls.snippet
local t = ls.text_node
local i = ls.insert_node

vim.keymap.set({ "i" }, "<C-K>", function()
    ls.expand()
end, { silent = true })
vim.keymap.set({ "i", "s" }, "<C-L>", function()
    ls.jump(1)
end, { silent = true })
vim.keymap.set({ "i", "s" }, "<C-J>", function()
    ls.jump(-1)
end, { silent = true })

vim.keymap.set({ "i", "s" }, "<C-E>", function()
    if ls.choice_active() then
        ls.change_choice(1)
    end
end, { silent = true })

ls.add_snippets("markdown", {
    s("frontmatter", {
        t({ "---", "title: " }),
        i(1, "Document Title"),
        t({ "", "date: " }),
        i(2, os.date("%Y-%m-%d")),
        t({ "", "tags: [" }),
        i(3, "tag1, tag2"),
        t({ "]", "---", "" }),
        i(0), -- Insert cursor at the end
    }),
})
```

- This change places the cursor at the end of the snippet after the frontmatter
  is inserted. You can trigger this snippet by typing frontmatter and then
  pressing `<C-K>` to expand it.
