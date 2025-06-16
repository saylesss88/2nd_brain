---
title: Absolute Permission Modes
aliases:
date: 2024-09-24
---

# Absolute Permission Modes

| mode | meaning                                             | Used for |
| ---- | --------------------------------------------------- | -------- |
| 644  | user:read/write;group, other:read                   | files    |
| 600  | user: read/write;group, other:none                  | files    |
| 755  | user: read/write/execute;group, other: read/execute | dirs     |
| 700  | user: read/write/execute;group, other: none         | dirs     |
| 711  | user: read/execute;group, other: none               | dirs     |

- Directories also have permissions. You can list the contents of a directory
  if it’s readable, but you can only access a file in a directory if the directory
  is executable.

- You need both in most cases; one common mistake when setting permissions of
  directories is accidentally remove the execute permission when using absolute
  modes.
