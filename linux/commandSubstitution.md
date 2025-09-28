---
title: Command Substitution
date: 2024-10-31
tags: [tag1, tag2]
---

# Command Substitution

One of the most useful features of shell scripts is the ablility to extract
information from the output of a command and assign it to a variable. After you
assign the output to a variable, you can use that value anywhere in your script.
This comes in handy when processing data in your scripts.

There are two ways to assign the output of a command to a variable:

- The backtick character (`)

- The $() format

You must either surround the entire command line command with two backtick
characters:

```bash
testing=`date`
```

or you can use the $() format.:

```bash
testing=$(date)
```

```bash
#!/usr/bin/bash
# copy the /usr/bin dir listing to a log file
today=$(date +%y%m%d)
ls /usr/bin -al > log.$today
```

- The log file appears in the directory using the value of the `$today`
  variable as part of the filename.

> [!CAUTION] Command substitution creates a _subshell_ to run the enclosed
> command. A subshell is a separate child shell generated from the shell that's
> running the script. Because of that, any variables you create in the script
> aren't available to the subshell command. Subshells are also created if you
> run a command from the command prompt using the `./path`, but they aren't
> created if you just run the command without a path. Shell built-ins don't
> generate a subshell.

[[redirection.md]]
