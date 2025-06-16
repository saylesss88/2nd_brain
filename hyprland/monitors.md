---
id: monitors
aliases: []
tags: []
---

The general config of a monitor looks like this:

```hyper Lang
monitor = name, resolution, position, scale
```

A common example:

```hypr Lang
monitor = DP-1, 1920x1080@144, 0x0, 1
```

  This will make the monitor on `DP-1` a `1920x1080`display, at 144hz, `0x0` off
  from the top left corner, with a scale of 1 (unscaled)

To list all available monitors (active and inactive):

```hyper Lang
hyprctl monitors all
```
