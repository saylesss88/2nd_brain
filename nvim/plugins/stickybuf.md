---
id: stickybuf
aliases:
  - Commands
tags: []
---

# Commands

  `PinBuffer[!]`: Pin the current buffer to the current window.

  `PinBufType[!]`: Pin the current buffer to the current window, but allow other
  buffers with the same buftype.

  `PinFiletype[!]`: Pin the current buffer to the current window, but allow
  other buffers with the same filetype.

  `Unpin`: Remove pinning for the current window.

## API

  `pin(winid, opts)`:
  Pin the buffer in the specified window.
