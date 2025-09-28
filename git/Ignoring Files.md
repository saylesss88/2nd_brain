---
id: Ignoring Files
aliases:
  - Ignoring Files
tags: []
---

# Ignoring Files

Often, you'll have a class of files that you don't want Git to automatically add
or even show you as being untracked. These are generally auto generated files
such as log files or files produced by your build system.

In such cases, you can create a file listing patterns to match them named
`.gitignore`. For example:

```bash
cat .gitignore
*.[oa]
*~
```

- The first line tells Git to ignore any files ending in ".o" or ".a".-- object
  and archive files that may be product of building your code.

- The second line tells Git to ignore all files whose names end with a tilde,
  which is used by many editors to mark temporary files.

Setting up a `.gitignore` file for your new repository before you get going is
generally a good idea so you don’t accidentally commit files that you really
don’t want in your Git repository.

The rules for the patterns you can put in the .gitignore file are as follows:
• Blank lines or lines starting with # are ignored.
• Standard glob patterns work, and will be applied recursively throughout the
entire working tree.
• You can start patterns with a forward slash (/) to avoid recursivity.
• You can end patterns with a forward slash (/) to specify a directory.
• You can negate a pattern by starting it with an exclamation point (!).

Glob patterns are like simplified regexs that shells use.

- An asterisk (\*) matches zero or more characters;
- `[abc]` matches any character inside the brackets
- A question mark (?) matches any single character
- `[0-9]` matches any single character between 0 and 9
- You can also use two asterisks to match nested directories; `a/**/z` would
  match `a/z`,`a/b/z`, `a/b/c/z`, etc.

Here's another example of a `.gitignore` file:

```bash
# ignore all .a files
*.a

# but do track lib.a, even though you're ignoring .a files above
!lib.a

# only ignore the TODO file in the current directory, not subdir/TODO
/TODO

# ignore all files in any directory named build
build/

# ignore doc/notes.txt, but not doc/server/arch.txt
doc/*.txt

# ignore all .pdf files in the doc/ directory and any of its subdirectories
doc/**/*.pdf

```

[[git diff.md]]
