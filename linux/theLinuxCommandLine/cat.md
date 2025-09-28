---
id: cat -- Concatenate Files
aliases: []
tags: []
---


The `cat` command reads one or more files and copies them to standard output
like so:

```bash
cat [file...]
```

You can use `cat` to display files without paging:

```bash
cat ls-output.txt
```

cat is often used to display
short text files. Since cat can accept more than one file as an argument, it can
also be used to join files together. Say we have downloaded a large file that
has been split into multiple parts (multimedia files are often split this way on
Usenet), and we want to join them back together. If the files were named
movie.mpeg.001 movie.mpeg.002 ... movie.mpeg.099
we could rejoin them with this command:

```bash
cat movie.mpeg.0* > movie.mpeg
```
- Since wildcards always expand in sorted order, the arguments will be
arranged in the correct order.

If you type `cat` without any arguments, cat copies stdin to stdout, so you see
your line of text repeated.

```bash
cat
The quick brown fox jumps over the lazy dog.
The quick brown fox jumps over the lazy dog.
```
- You can use this behavior to create short text files. If we wanted to do this
  with the above example:

  ```bash
  cat > lazy_dog.txt
  The quick brown fox jumps over the lazy dog.
  ```

Now that we know how cat accepts stdin in addition to filename arguments, let's
try redirecting standard input:

```bash
cat < lazy_dog.txt
STDIN
The quick brown fox jumps over the lazy dog.
```
- Using the `<` operator, we can redirect standard input from the keyboard to
  the file lazy_dog.txt.

- This is not particularly useful
compared to passing a filename argument, but it serves to demonstrate
using a file as a source of standard input. Other commands make better
use of standard input, as we shall soon see.

[[Pipelines.md]]
