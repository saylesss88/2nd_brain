---
id: render-markdown
aliases:
  - render-markdown
tags: []
---

# render-markdown

## lazy.nvim

```lua
return {
    'MeanderingProgrammer/markdown.nvim',
    name = 'render-markdown', -- Only needed if you have another plugin named markdown.nvim
    dependencies = {
        'nvim-treesitter/nvim-treesitter', -- Mandatory
        'nvim-tree/nvim-web-devicons', -- Optional but recommended
    },
    config = function()
        require('render-markdown').setup({})
    end,
}
```

### Commands

`:RenderMarkdownToggle`

- Functions can also be accessed directly through `require('render-markdown').toggle()`
