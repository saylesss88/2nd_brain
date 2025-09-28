---
title: Wallust
date: 2024-11-28
tags: [tag1, tag2]
---

# Wallust

```bash
wallust run Anime-Room.png --backend full
```

- Changes terminal colors to match the image.

## Backend

| Backends   | Description                                           |
| ---------- | ----------------------------------------------------- |
| Full       | Read and return the whole image pixels (slower)       |
| Resized    | Resizes the image before parsing, faster              |
| Wal        | Uses image magick `convert` to generate the colors    |
| Thumb      | Faster algo hardcoded to 512x512 (no ratio respected) |
| FastResize | Much faster resize algo                               |
| Kmeans     | divides and picks pixels all around the image         |

- Config file: `backend = "full"`
- Cli: `wallust run image.png --backend full`

## Customization

```bash
wallust run image.png --colorspace lchmixed
```

Lab Uses Cie Lab color space
LabMixed Variant of lab that mixes the colors gathered, if not enough colors it fallbacks to usual lab (not recommended in small images)
Lch CIE Lch, you can understand this color space like LAB but with chrome and hue added. Could help when sorting.
LchMixed CIE Lch, you can understand this color space like LAB but with chrome and hue added. Could help when sorting.
LchAnsi Variant of Lch which preserves 8 colors: black, red, green, yellow, blue, magenta, cyan and gray. This works best with 'darkansi' palette, allowing a constant color order.
