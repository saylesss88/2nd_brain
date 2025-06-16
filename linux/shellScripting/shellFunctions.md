---
id: shellFunctions
aliases:
  - shellFunctions
tags: []
---

# shellFunctions

A function is written in much the same way as a shell script but is different in
that it is defined, or written, within a shell script most of the time, and is
called within the script.

We can define functions at the system level that is always available in our
environment.

A function has the following form:

```bash
function function_name
{
    commands to execute
```
or

```bash
function_name ()
{
    commands to execute
}
```

