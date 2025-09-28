---
id: Disposing of Unwanted Output
aliases:
  - Disposing of Unwanted Output
tags: []
---

# Disposing of Unwanted Output

This applies particularly to error and
status messages. The system provides a way to do this by redirecting output
to a special file called /dev/null. This file is a system device called a bit bucket,
which accepts input and does nothing with it. To suppress error messages
from a command, we do this:

```bash
ls -l /bin/usr 2> /dev/null
```
> /dev/null (bit bucket) is an ancient Unix concept, and due to its universality
> it has appeared in many parts of Unix culture.

[[cat -- Concatenate Files.md]]
