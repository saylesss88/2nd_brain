---
id: streamEditor
aliases:
  - streamEditor
  - streamEditor (sed)
tags: []
---

# streamEditor (sed)

The `sed` command is a simple scriptable editor, so it can perform only simple
edits, such as removing lines that have text matching a certain pattern,
replacing one pattern of characters with another, etc.

Here the sed command searches the
entire /etc/passwd file, searches for the word home, and prints any line containing the
word home:

```bash
sed -n '/home/p' /etc/passwd
jr:x:1000:1000:jr:/home/jr:/bin/zsh
```

In this next example, sed searches the file somefile.txt and replaces every instance of
the string Mac with Linux. Notice that the letter g is needed at the end of the substitu-
tion command to cause every occurrence of Mac on each line to be changed to Linux.
(Otherwise, only the first instance of Mac on each line is changed.) The output is then sent
to the fixed_file.txt file. The output from sed goes to stdout, so this command redi-
rects the output to a file for safekeeping.

```bash
sed 's/Mac/Linux/g' somefile.txt > fixed_file.txt
```

You can get the same result using a pipe:

```bash
cat somefile.txt | sed 's/Mac/Linux/g' > fixed_file.txt
```

By searching for a pattern and replacing it with a null pattern, you delete the original
pattern. This example searches the contents of the somefile.txt file and replaces extra
blank spaces at the end of each line (s/ \*$) with nothing (//). Results go to the fixed\_
file.txt file.

```bash
cat somefile.txt | sed 's/ *$//' > fixed_file.txt
```
