---
id: Filters
aliases: []
tags: []
---

## Filters

Pipelines are often used to perform complex operations on data. It is pos-
sible to put several commands together into a pipeline. Frequently, the com-
mands used this way are referred to as filters. Filters take input, change it
somehow, and then output it.

Imagine we want to make a combined list of all of the executable programs in
`/bin` and `/usr/bin`, put them in sorted order, and then view the list:

```bash
ls /bin /usr/bin | sort | less
```

- Since we have specified two directories, the output of `ls` would have
  consisted of two sorted lists, one for each directory.

## uniq -- Report or Omit Repeated Lines

The `uniq` command is often used in conjunction with `sort`.

- `uniq` accepts a sorted list of data from either standard input or a single
  filename argument and by default, removes any duplicates from the list.

 So, to make sure our list has no duplicates (that is, any pro-
grams of the same name that appear in both the /bin and /usr/bin director-
ies) we will add uniq to our pipeline:

```bash
ls /bin /usr/bin | sort | uniq | less
```
- In this example, we use `uniq` to remove any duplicate lines from the output
  of the `sort` command.

If we want to see a list of the duplicates instead:

```bash
ls /bin /usr/bin | sort | uniq -d | less
```

### wc -- Print Line, Word, and Byte Counts

The `wc` (word count) command is used to display the number of lines, words, and
bytes contained in files:

```bash
wc ls-output.txt
```
```bash
 ╰─λ wc ls-output.txt 
  3835  29265 568557 ls-output.txt
```

In this case it prints out three numbers: lines, words, and bytes con-
tained in ls-output.txt. Like our previous commands, if executed without
command-line arguments, wc accepts standard input. The -l option limits
its output to only report lines. Adding it to a pipeline is a handy way to
count things. To see the number of items we have in our sorted list, we
can do this:

```bash
ls /bin /usr/bin | sort | uniq | wc -l
3839
```

[[grep.md]]
