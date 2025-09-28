---
title: Print from CLI
date: 2024-10-28
tags: [tag1, tag2]
---

1. Locate your printer:

```bash
lpstat -p
```

2. View print queues:

```bash
lpstat -p | awk '{print $2}' | xargs -n1 lpq -P
```

3. Print zdnet.txt

```bash
lp zdnet.txt
```
