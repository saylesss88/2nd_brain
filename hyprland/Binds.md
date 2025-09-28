---
id: Binds
aliases: []
tags: []
---

## Basic

```hyperlang
bind = MODS, key, dispatcher, params
```

for example,

```hyper Lang
bind = SUPER_SHIFT, Q, exec, firefox
```

will bind opening Firefox to SUPER + SHIFT + Q

> [!] For binding keys without a modkey, leave it empty:
> `bind = , Print, exec, grim`

[[List of Dispatchers.md]]
