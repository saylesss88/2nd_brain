---
title: Escaping Characters
date: 2024-10-11
tags: [bash, tag2]
---

```bash
echo "The balance for user $USER is: \$5.00"
The balance for user $USER is: $5.00
```

It is common to use excaping to eliminate the special meaning of characters in a
filename.

To include a special character in a filename, you can do this:

```bash
mv bad\&filename good_filename
```

- To allow a backslash character to appear, escape it by typing `\\`. Within
  single quotes, the backslash character is not special.

[[commandlineediting.md]]
