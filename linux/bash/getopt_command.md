---
id: getopt_command
aliases:
  - `getopt` command
tags:
  - bash
  - commands
---

# `getopt` command

`getopt` reorganizes the command line parameters to make parsing them in your
script easier.

**Syntax:**

`getopt optstring parameters`

- The `optstring` is the key to the process. It defines the valid option letters
  that can be used in the command line. It also defines which option letters
  require a parameter value.

First, list each command line option letter you're going to use in your script
in the `optstring`. Then place a colon after each option letter that requires a
parameter value. The `getopt` command parses the supplied parameters based on
the `optstring` you define.

**Examples:**

```bash
getopt ab:cd -a -b test1 -cd test2 test3
```

Output:
`-a -b test1 -c -d -- test2 test3`

- The `optstring` defines four valid option letters, `a`, `b`, `c` and `d`. A
  colon is placed behind the letter `b` in order to require a parameter value.

- When the `getopt` command runs, it examines the provided parameter list (-a -b
  test1 -cd test2 test3) and parses them based on the `optstring` you define.
