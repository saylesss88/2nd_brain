---
id: redirectingErrors
aliases:
  - Redirecting Errors
tags:
  - bash
  - redirection
---

# Redirecting Errors

The `STDERR` file discriptor is set to the value `2`. You can select to redirect
only error messages by placing this file descriptor value immediately before the
redirection symbol. The value must appear immediately before the redirection
symbol or it doesn't work:

```bash
ls -al badfile 2> test4
cat test4
ls: cannot access badfile: No such file or directory
```

- Now when you run the command, the error message is redirected to a file. Using
  this method, the shell redirects error messages only, not the normal data.

An example of mixing STDOUT and STDERR:

```bash
ls -al test badtest test2 2> test5
-rw-rw-r-- 1 rich 158 2014-10-16 11:32 test2
cat test5
ls: cannot access test: No such file or directory
ls: cannot access badtest: No such file or directory
```

- The normal `STDOUT` output from the `ls` command still goes to the default
  `STDOUT` file descriptor, which is the monitor.Because the command redirects ﬁle descriptor 2 output
  (STDERR) to an output file, the shell sends any error messages generated directly to the
  speciﬁed redirection file.

## Redirecting Errors and Data

```bash
ls -al test test2 test3 badtest 2> test6 1> test7
cat test6
ls: cannot access test: No such file or directory
ls: cannot access badtest: No such file or directory
$ cat test7
-rw-rw-r-- 1 rich rich 158 2014-10-16 11:32 test2
-rw-rw-r-- 1 rich rich
0 2014-10-16 11:33 test3
```

The shell redirects the normal output of the ls command that would have gone to STDOUT
to the test7 file using the 1> symbol. Any error messages that would have gone to
STDERR were redirected to the test6 file using the 2> symbol.

- You can use this technique to separate normal script output from error
  messages that occur in the script. This allows you to easily identify errors
  without having to wade through thousands of lines of normal script output.

Alternatively, if you want, you can redirect both STDERR and STDOUT output to the same
output file. The bash shell provides a special redirection symbol just for this purpose, the
&> symbol:

```bash
$ ls -al test test2 test3 badtest &> test7
$ cat test7
ls: cannot access test: No such file or directory
ls: cannot access badtest: No such file or directory
-rw-rw-r-- 1 rich rich 158 2014-10-16 11:32 test2
-rw-rw-r-- 1 rich rich   0 2014-10-16 11:33 test3
```
