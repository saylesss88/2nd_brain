---
id: Connecting&ExpandingCommands
aliases: []
tags:
  - commands
---

A truly powerful feature of the shell is the capability to redirect the input
and output of commands to and from other commands and files.

- To do this the shell uses *metacharacters* which have special meaning to the
  shell for connecting commands or requesting expansion.

   - Metacharacters include the pipe character (`|`), ampersand (`&`), semicolon
     (`;`), right parenthesis (`)` left parenthesis (`(`), less than (`<`), and
     greater than (`>`).

  ## Piping between commands

  The pipe (`|`) metacharacter connects the output of one command to the input
  of another command. This lets you have one command work on some data and then
  have the next command deal with the results.

```bash
cat /etc/passwd | sort | less
```

- This command lists the contents of the file `/etc/passwd` and pipes the output
  to the `sort` command.

  - The `sort` command takes the usernames that begin each line of the file,
    sorts them alphabetically, and then pipes the output to the `less` command.

    Pipes are an excellent illustration of how UNIX, the predecessor of Linux, was created as an
operating system made up of building blocks. A standard practice in UNIX was to connect
utilities in different ways to get different jobs done. For example, before the days of graphical
word processors, users created plain-text files that included macros to indicate formatting. To
see how the document really appeared, they would use a command such as the following:

```bash
gunzip < /usr/share/man/man1/grep.1.gz | nroff -c -man | less
```

- In this example, the contents of the grep man page(grep.1.gz) are directed to
  the gunzip command to be unzipped. The output of the gunzip command is then
  piped to the `nrof` command to format the man page using the manual macro
  (-man).

- To display the output, it is piped to the `less` command.

[[SequentialCommands.md]]
