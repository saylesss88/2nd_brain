---
id: ENV
aliases:
  - ENV
tags: []
---

# ENV

All key-value pairs in the `[env]` section will be added as environment
variables for any process spawned by Alacritty, including its shell.

Example:

```toml
[env]
WINIT_X11_SCALE_FACTOR = "1.0"
```
