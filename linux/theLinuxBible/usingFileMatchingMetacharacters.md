---
id: usingFileMatchingMetacharacters
aliases:
  - Using File Matching Metacharacters
tags:
  - files
---

# Using File Matching Metacharacters

To save a few keystrokes and enable you to refer easily to a group of files, the
bash shell lets you use metacharacters.

- Any time you need to refer to a file or directory, such as to list, open, or
  remove it, you can use metacharacters to match the files you want.

Here are some useful metacharacters:

- `*` : Matches any number of characters.

- `?` : Matches any single character.

- `[...]` : Matches any single character between the brackets, which can include
  a hyphen-separated range of letters or numbers.

`touch apple banana grape grapefruit watermelon`

```bash
ls a*   # matches any file that begins with a
apple
ls g*t  # matches any file that begins with g and ends with t
grapefruit
ls *e*  # matches any file that contains e
apple grape grapefruit watermelon
```

```bash
ls ????e # match any five-char file that ends with e
apple grape
ls g???e* # any file that begins with g, & has an e at 5th position
grape grapefruit
```

The following examples use braces to do pattern matching:

```bash
ls [abw] * # files that beginn with a, b, or w
apple banana watermelon
ls [agw] * [ne] # files that beginn with a, g, or w, & end with n or e
apple grape watermelon
ls [a-g] * # files that beginn with a, b, c, d, e, f, g
apple banana grape grapefruit
```
[[fileRedirection.md]]
