---
id: ExpandingCommands
aliases:
  - Expanding Commands
tags:
  - commands
---
# Expanding Commands

With command substitution, you can have the output of a command interpreted by
the shell instead of by the command itself.  In this way, you can have the
stdout of a command become an argument to another command.

- The two forms of command substitution are `$(command)` and `command`
  (backticks, not single quotes).

The command in this case can include options, metacharacters, and arguments. The
following is an example of using command substitution:

```bash
vi $(find /home | grep xyzzy)
```

- In this example, the command substitution is done before the `vi` command is
  run.

   - First, the `find` command starts at the `/home` directory and prints out
     all of the files and directories below that point in the filesystem.

    - The output is then piped to the `grep` command, which filters out all
      files except for those that include the word `xyzzy` in the filename.

    - Finally, the `vi` command opens all filenames for editing (one at a time)
      that include `xyzzy`

> This particular example is useful if you want to edit a file for which you
> know the name but not the location. As long as the string is uncommon, you can
> find and open every instance of a filename existing beneath a point you
> choose in the filesystem. Dont use `grep` from the root filesystem or you'll
> match and try to edit several thousand files.

## Expanding Arithmetic Expressions

Sometimes you want to pass arithmetic results to a command.  There are two forms that
you can use to expand an arithmetic expression and pass it to the shell: $[expression]
or $(expression). The following is an example:

```bash
echo "I am $[2020 - 1957] years old."
I am 63 years old.
```

- The shell interprets the arithmetic expression first (2020 - 1957) and then
  passes that info to the `echo` command. The `echo` command prints out the
  results of the arithmetic expression (63).

Here;s an example of the other form:

```bash
echo "There are $(ls | wc -w) files in this directory."
There are 14 files in this directory.
```

- This lists the contents of the current directory using the `ls` command and
  runs the word count command to count the number of files found (`wc -w`). The
  resulting number (14 in this case) is echoed back with the rest of the
  sentence shown.

### Expanding Variables

Variable that store info within the shell can be expanded using the dollar sign
(`$`) metacharacter. When you expand an environment variable on the command
line, the value of the variable is printed instead of the variable name itself,
as follows:

```bash
ls -l $BASH
-rwxr-xr-x. 1 root root 1219248 Oct 12 17:59 /usr/bin/bash
```

- Using `$BASH` as an argument to `ls -l` causes a long list of the bash command
  to be printed.

[[UsingShellVariables.md]]
