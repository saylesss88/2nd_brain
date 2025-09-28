---
id: bareBonesVim
aliases:
  - bareBonesVim
tags: []
---

# bareBonesVim

Add this to a file named `essential.vim`:

```vim essential.vim
set nocompatible
filetype plugin on
```

When launching Vim, you can use this file instead of your `~/.vimrc` file by
running the following:

```bash
vim -u jr/essential.vim
```


