---
id: permanentRedirections
aliases:
  - Permanent redirections
tags: []
---

# Permanent redirections

With the `exec` command you can tell the shell to redirect specific file
descriptor for the duration of the script:

```bash
#!/usr/bin/bash
# redirecting all output to a file
exec 1>testout
echo "This is a test of redirecting all output"
echo "from a script to another file."
echo "without having to redirect every individual line"
```

```bash
$ ./test10
$ cat testout
This is a test of redirecting all output
from a script to another file.
without having to redirect every individual line
```

- The `exec` command starts a new shell and redirects the `STDOUT` file
  descriptor to a file. All output in the script that goes to `STDOUT` is
  redirected to the file.

You can also redirect the `STDOUT` in the middle of a script:

```bash
#!/usr/bin/bash
# redirecting output to different locations
exec 2>testerror
echo "This is the start of the script"
echo "now redirecting all output to another location"
15
exec 1>testout
echo "This output should go to the testout file"
echo "but this should go to the testerror file" >&2
```
