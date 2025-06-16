---
id: writingSimpleShellScripts
aliases: []
tags: []
---

You can execute a shell script in two basic ways:

- `bash myscript` In this
  method, the file does not need to be executable; it just contains a list of shell com-
  mands. The shell specified on the command line is used to interpret the commands
  in the script file. This is most common for quick, simple tasks

- You can add a shebang line to the top of your script and `chmod +x myscript`
  to make it executable.

When scripts are executed either way, options for the program may be specified
on the command line.

- Anything following the name of the script is referred to as a command line
  argument.

[[shellVariables.md]]
