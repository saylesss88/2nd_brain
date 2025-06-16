---
id: Redirecting Standard Error
aliases:
  - Redirecting Standard Error
tags: []
---

# Redirecting Standard Error

When redirecting standard error there is no dedicated redirection operator.

- To redirect standard error we must refer to its *file descriptor*.

- We have referred to the first three file streams as standard input, output,
  and error. The shell references these as `0`, `1`, and `2` respectively.

Since standard error is the same as file descriptor `2` we can redirect to
standard error with this notation:

```bash
ls -l /bin/usr 2> ls-error.txt
```
- The file descriptor `2` is placed immediately before the redirection operator
  to perform the redirection of standard error to the file `ls-error.txt`.

## Redirecting Standart Output & Standard Error to One File

There are two ways to do this. Here is the traditional way, which works with old
versions of the shell:

```bash
ls -l /bin/usr > ls-output.txt 2>&1
```

```bash
╭─jr@jr in repo: 2nd_brain/linux on  main [!?] took 11s
 ╰─λ ls -l ls-output.txt 
.rw-r--r-- 51 jr 27 Aug 13:37  ls-output.txt
```

Using this method, we perform two redirections. First we redirect
standard output to the file ls-output.txt, and then we redirect file descriptor
2 (standard error) to file descriptor 1 (standard output) using the nota-
tion 2>&1 .

> [!NOTE] Notice that the order of the redirections is significant. The redirection of standard error
must always occur after redirecting standard output or it doesn’t work. In the example
above, > ls-output.txt 2>&1 redirects standard error to the file ls-output.txt, but if
the order is changed to 2>&1 > ls-output.txt, standard error is directed to the screen.

Recent versions of bash provide a second, more streamlined method for performing
this combined redirection:

```bash
ls -l /bin/usr &> ls-output.txt
```

In this case, we use the single notation `&>` to redirect both standard output
and standard error to the file ls-output.txt.

[[Disposing of Unwanted Output.md]]
