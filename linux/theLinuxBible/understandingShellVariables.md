---
id: understandingShellVariables
aliases:
  - Special shell positional parameters
tags:
  - variables
---

# Special shell positional parameters

There are special variables that the shell assigns for you. One set of commonly
used variables is calles _positional parameters_ or _command-line arguments_,
and it is referenced as `$0`, `$1`, `$2`, etc.

- `$0` is special, and it is assigned the name used to invoke your script; the
  others are assigned the values of the parameters passed on the command line in
  the order they appeared.

Lets say you had a script named `myscript` in your `/home/user/bin/myscript`
which contains the following:

```bash
#!/bin/bash
# Script to echo out command-line arguments
echo "The first argument is $1, the second is $2."
echo "The command itself is called $0."
echo "There are $# parameters on your command line"
echo "Here are all the arguments: $@"
```

Assuming that the script is executable and located in a directory in your $PATH,
the following shows what happens when you run `myscript`:

```bash
chmod 755 /home/jr/bin/myscript
myscript foo bar
The first argument is foo, the second is bar.
The command itself is called /home/chris/bin/myscript.
There are 2 parameters on your command line
Here are all the arguments: foo bar
```

- As you can see, the positional parameter $0 is the full path or relative path to myscript,
  $1 is foo, and $2 is bar.

- Another variable, `$#` is the number of parameters your script was given. In
  the example `$#` would be 2.

- The `$@` variable holds all of the arguments entered at the command line.

- Another useful shell variable is `$?`, which holds the exit status of the last
  command executed. Typically a value of zero means that the command was
  successful, and anything other than zero means that something went wrong.

[[readingInParameters.md]]
