---
id: fileRedirection
aliases:
  - Using file-redirection metacharacters
tags:
  - files
---

# Using file-redirection metacharacters

Commands receive data from standard input and send it to standard output. Using
pipes, you can direct standard output from one command to the standard input
ofanother.

- `<` : Directs the contents of a file to the command. In most cases, this is
  the default action expected by the command and the use of the character is
  optional; using `less bigfile` is the same as `less < bigfile`.

- `>` : Directs the output of a command to a file. If the file exists, the
  content is overwritten.

- `2>` : Directs standard error to the file.

- `&>` : Directs both standard output and standard error to the file.

- `>>` : Appends the output of a command to the end of a file.

Here are some examples of command lines where info is directen to and from
files:

```bash
mail root < ~/.bashrc
man chmod | col -b > /tmp/chmod
echo "I finished the project on $(date)" >> ~/.bashrc
```

- In the first example, the content of the `.bashrc` file in the home dir is
  sent in a mail message to the computer's root user.

- The second line formats the `chmod` man page, removes extra back spaces (`col
-b`), and sends the output to the `/tmp/chmod` file.

- Another type of redirection, referred to as _here text_ (also called _here
  document_), enables you to type text that can be used as input to a command.

  - Here documents involve entering two less-than characters (`<<`) after a
    command, followed by a word.

  - All typing folling that word is taken as user input until the word is
    repeated on a line by itself.

Here is an example:

```bash
$ mail root cnegus rjones bdecker << thetext
> I want to tell everyone that there will be a 10 a.m.
> meeting in conference room B. Everyone should attend.
>
> -- James
> thetext
$
```

- This example sends a mail message to root, cnegus, rjones, and bdecker
  usernames.

  - The text entered between `<<thetext` and `thetext` becomes the content of
    the message.

  - A common use of here text is to use it with a text editor to create or add
    to a file from within a script:

```bash
/bin/ed /etc/resolv.comf <<resendit
a
nameserver 100.100.100.100
.
w
q
resendit
```

- With these lines added to a script run by the root user, the ed text editor
  adds the IP address of a DNS server to the `/etc/resolv.conf` file.

## Using brace expansion characters

By using curly braces (`{}`), you can expand out a set of characters across
filenames, directory names, or other arguments which you give commands.

For example, to create a set of files from `memo1` through `memo5`:

```bash
touch memo{1,2,3,4,5}
ls
memo1 memo2 memo3 memo4 memo5
```

- The items that are expanded don't have to be numbers or even single digits.
  You could also use any string of characters, as long as you separate them with
  commas:

```bash
$ touch {John,Bill,Sally}-{Breakfast,Lunch,Dinner}
$ ls
Bill-Breakfast Bill-Lunch John-Dinner Sally-Breakfast Sally-Lunch
Bill-Dinner John-Breakfast John-Lunch Sally-Dinner
$ rm -f {John,Bill,Sally}-{Breakfast,Lunch,Dinner}
$ touch {a..f}{1..5}
$ ls
a1 a3 a5 b2 b4 c1 c3 c5 d2 d4 e1 e3 e5 f2 f4
a2 a4 b1 b3 b5 c2 c4 d1 d3 d5 e2 e4 f1 f3 f5
```

- In the first example, the use of two sets of braces means John, Bill, and
  Sally each have filenames associated with Breakfast, Lunch, and Dinner.

  - If you make a mistake, its easy to recall the command and change touch to
    `rm -f` to remove the files.

[[listingFiles&Directories.md]]
