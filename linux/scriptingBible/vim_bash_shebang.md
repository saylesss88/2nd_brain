---
id: vim_bash_shebang
aliases:
  - Boilerplate macro
tags: []
---

# Boilerplate macro

```vim
nnoremap bs i#!/usr/bin/bash/<ESC>o
```

```vim
nnoremap bs i#!/bin/bash/<ESC>o<ESC>ofunction main(){<ESC>o<ESC>o}<ESC>ki<S-TAB>
```

## Sourcing Scripts

```vim
nnoremap sh :!chmod +x && source %
```
