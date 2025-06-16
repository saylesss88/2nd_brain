---
id: Displaying the last part of files using tail
aliases:
  - Displaying the last part of files using tail
tags: []
---

# Displaying the last part of files using tail

If you want to display a different number of lines than the default of 10, you
can use the `-n` flag.

```bash
tail -n 50 file.txt
```

>Tip: Another usage of the `tail` command is its follow-along (`-f`) option.
This option enables you to follow a file as it is being written. This is useful
for viewing and monitoring files in real-time.

[[wc command.md]]
