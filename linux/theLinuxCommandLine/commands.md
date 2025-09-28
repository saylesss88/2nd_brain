---
id: commands
aliases: []
tags: []
---


To copy all the HTML files from one directory to another -- but only those that
do not exist in the destination directory or are newer than the versions in the
  destination directory:

```bash
cp -u *.html destination
```

 Because the shell uses file-
names so much, it provides special characters to help you rapidly specify
groups of filenames. These special characters are called wildcards. Using
wildcards (also known as globbing) allows you to select filenames based on
patterns of characters. Table 4-1 lists the wildcards and what they select.
Table 4-1: Wildcards
Wildcard
 Matches
`*`                Any characters
?                  Any single character
[characters]        Any character that is a member of the set characters
[!characters]       Any character that is not a member of the set characters
[[:class:]]         Any character that is a member of the specified class
Table 4-2 lists the most commonly used character classes.

Table 4-2: Commonly Used Character Classes
Character Class     Matches
[:alnum:]             Any alphanumeric character

[:alpha:]             Any alphabetic character

[:digit:]             Any numeral

[:lower:]             Any lowercase letter

[:upper:]           Any uppercase letter

Examples:

**Command**                    **Output**

cp file1 file2           Copy file1 to file2. If file 2 exists, it is
                         overwritten, if it does not exist, it is created.

cp -i file1 file2        Same as above, but if file 2 exists, the user is
                         prompted before overwriting it.

cp file1 file2 dir1      Copy file1 to file2 in directory dir1.

cp dir1/* dir2           Copy all files in directory dir1 to directory dir2.
                         dir2 must exist.

cp -r dir1 dir2          Copy directory dir1 (and all its contents)to directory dir2.

[[Move and Rename Files.md]]
