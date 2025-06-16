---
id: Redirection
aliases:
  - Redirection
tags: []
---

# Redirection

I/O redirection. The I/O stands for input/output, and with this
facility you can redirect the input and output of
commands to and from files, as well as connect multiple commands to
make powerful command pipelines. To show off this facility, we will intro-
duce the following commands:

- `cat`—Concatenate files.

- `sort`—Sort lines of text.

- `uniq`—Report or omit repeated lines.

- `wc`—Print newline, word, and byte counts for each file.

- `grep`—Print lines matching a pattern.

- `head`—Output the first part of a file.

- `tail`—Output the last part of a file.

- `tee`—Read from standard input and write to standard output and files.

## Standard Input, Output, and Error

Many of the programs that we have used so far produce output of some
kind. This output often consists of two types. First, we have the program’s
results; that is, the data the program is designed to produce. Second, we
have status and error messages that tell us how the program is getting along.

- If we look at a command like `ls`, we will see that it displays the results
  and its error messages on the screen.

- Keeping with the Unix theme of "everything is a file", programs such as `ls`
  actually send their results to a special file called *standard output*.(often
  expressed as `stdout`) and their status messages to another file called
  *standard error (`stderr`). By default, both standart output and standard
  error are linked to the screen and not saved into a disk file.

- In addition, many programs take input from a facility called standard input
(stdin), which is, by default, attached to the keyboard.

- I/O redirection allows us to change where output goes and where input comes
  from. Normally output goes to the screen and input comes from the keyboard,
  but with I/O redirection we can change that.

## Redirecting Standard Output

- To redirect standard output to a file, we use the `>` operator.

```bash
ls -l /usr/bin > ls-output.txt
```
- Here, we created a long listing of the `/usr/bin` directory and sent the
  results to the file `ls-output.txt`.

Lets examine the redirected output of the command:

```bash
ls -l ls-output.txt
.rw-r--r-- 569k jr 27 Aug 09:49  ls-output.txt
```

With the `>` operator, the file is always overwritten.

```bash
ls -l /bin/usr > ls-output.txt
```

Running `ls -l ls-output.txt` shows that the file is empty.
Output:
`-rw-rw-r-- 1 me me 0 2012-02-01 15:08 ls-output.txt`

- `/bin/usr` doesn't exist but if you check `ls -l ls-output.txt`, you will see
  that it erased our previous output.

- This is because the `ls` program does not send its error messages to `stdout`
  Instead, like most well-written Unix programs, it sends its error messages to
  standard error.

- Since we redirected standard output not standard error, the error message was
  still sent to the screen.

Since our ls command generated no results and only an
error message, the redirection operation started to rewrite the file and then
stopped because of the error, resulting in its truncation. In fact, if we ever
need to actually truncate a file (or create a new, empty file) we can use a
trick like this:

```bash
> ls-output.txt
```
- Simply using the redirection operator with no command preceding it will
  truncate an existing file or create a new, empty file.

To append redirected output to a file, we use the `>>` operator.

```bash
ls -l /usr/bin >> ls-output.txt
```
