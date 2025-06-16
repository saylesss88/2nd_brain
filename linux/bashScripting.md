---
id: Bash
aliases:
  - How to Create and Execute Bash Scripts
tags: []
---

# How to Create and Execute Bash Scripts

By naming convention, bash scripts end with `.sh`. However they aren't required.

## Adding the Shebang

Bash scripts start with a `shebang`. Shebang is a combination of `bash #` and
`bang !` followed by the bash shell path. It tells the shell to execute it via
the bash shell. Shebang is simply an absolute path to the bash interpreter.

example:

```bash
#!/usr/bin/env bash
```

### Creating your first bash script

Create a file named `run_all.sh`
`vim run_all.sh`

```bash
#!/bin/bash
echo "Today is " `date`

echo -e "\nenter the path to directory"
read the_path

echo -e "\n you path has the following files and folders: "
ls $the_path

```

### Executing your bash script

To make the script executable, you can use the `chmod` command:

```bash
chmod u+x run_all.sh
```

- `chmod` modifies the ownership of a file for the current user :`u`.
- `+x` adds the execution rights to the current user. This means that the user
  who is the owner can now run the script.
- `run_all.sh` is the name of the script.

You can run the script using any of the following commands:

- `sh run_all.sh`
- `./run_all.sh`
- `bash run_all.sh`

[[Variables and data types in Bash.md]]
