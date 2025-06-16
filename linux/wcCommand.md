---
id: wc command
aliases:
  - wc command
tags: []
---

# wc command

You can count words, lines and characters in a file using the `wc` command.

For example, running `wc syslog.log` gave the output:

```bash
1669 9623 64367 syslog.log
```

- `1669` is the number of lines in the file
- `9623` is the number of words in the file
- `64367` is the number of characters in the file

- [!] So, the command `wc syslog.log` counted `1669` lines, `9623` words and
  `64367` characters

## Comparing files line by line using `diff`

Comparing and finding differences between two files is a common task in Linux.
You can compare two files right within the command line using the `diff` command.

The basic syntax of the `diff` command is:

```bash
diff [options] file1 file2
```

Here are two files, hello.py and also-hello.py, that we will compare using the

diff command:

```bash
# contents of hello.py

def greet(name):
    return f"Hello, {name}!"

user = input("Enter your name: ")
print(greet(user))

```

```bash
# contents of also-hello.py

more also-hello.py
def greet(name):
    return fHello, {name}!

user = input(Enter your name: )
print(greet(user))
print("Nice to meet you")

```

1. Check whether the files are the same or not

```bash
diff -q hello.py also-hello.py
# Output
Files hello.py and also-hello.py differ
```

2. See how the files differ with the `-u`(unified) flag:

```bash
diff -u hello.py also-hello.py
--- hello.py    2024-05-24 18:31:29.891690478 +0500
+++ also-hello.py    2024-05-24 18:32:17.207921795 +0500
@@ -3,4 +3,5 @@

 user = input(Enter your name: )
 print(greet(user))
+print("Nice to meet you")

```

- `--- hello.py 2024-05-24 18:31:29.891690478 +0500` indicates the file being
  compared and its timestamp

- `+++ also-hello.py 2024-05-24 18:32:17.207921795 +0500` indicates the other
  file being compared and its timestamp

- `@@ -3,4 +3,5 @@` shows the line numbers where the changes occur. In this case
  it indicates that lines 3 and 4 in the original file have changed to lines 3
  and 5 in the modified file
