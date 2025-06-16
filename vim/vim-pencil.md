---
id: vim-pencil
aliases: []
tags: []
---

You can manually enable, disable, and toggle pencil as a command:

Pencil - initialize pencil with auto-detect for the current buffer
NoPencil (or PencilOff) - removes navigation mappings and restores buffer to global settings
TogglePencil (or PencilToggle) - if on, turns off; if off, initializes with auto-detect

Because auto-detect might not work as intended, you can invoke a command to set the behavior for the current buffer:

SoftPencil (or PencilSoft) - initialize pencil with soft line wrap mode
HardPencil (or PencilHard) - initialize pencil with hard line break mode (and Vim’s autoformat)
