---
title: Copying Files
date: 2024-11-26
tags: [bash, copy]
---

# Copying Files

When both the source and destination parameters are filenames, the `cp` command
copies the source file to a new destination file. The new file acts like a brand
new file, with an updated modification time:

```bash
cp test_one test_two
ls -l test_*
-rw-rw-r-- 1 christine christine 0 May 21 14:35 test_one
-rw-rw-r-- 1 christine christine 0 May 21 15:15 test_two
```


