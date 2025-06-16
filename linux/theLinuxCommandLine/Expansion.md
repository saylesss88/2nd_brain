---
id: Expansion
aliases: []
tags: []
---

# Expansion

Each time you type a command line and press the ENTER key, bash performs several
processes upon the text before it carries out your command.

- We’ve seen a couple of cases of how a simple character sequence, for example \*,
  can have a lot of meaning to the shell. The process that makes this happen
  is called expansion. With expansion, you enter something, and it is expanded
  into something else before the shell acts upon it.

To demonstrate what is meant by this, let's look at the echo command. `echo` is
a shell builtin that prints its arguments to standard output.

```bash
echo this is a test
this is a test
```

the shell expands the _ into something
else (in this instance, the names of the files in the current working direct-
ory) before the echo command is executed. When the ENTER key is pressed,
the shell automatically expands any qualifying characters on the command
line before the command is carried out, so the echo command never saw
the _, only its expanded result. Knowing this, we can see that echo behaved
as expected.

## Pathname Expansion

The mechanism by which wildcards work is called _pathname expansion_.

Given a home directory looks like this:

```bash
ls
Desktop
 ls-output.txt Pictures
 Templates
Documents Music
 Public
 Videos
```

we could carry out the following expansions:

```bash
echo D*
Desktop Documents
```

and

```bash
echo *s
Documents Pictures Templates Videos
```

or even:

```bash
echo [[:upper:]]*
Documents Pictures Templates Videos Music Videos Public
```

And looking beyond our home directory:

```bash
echo /usr/*/share
/usr/local/share
```

To show all the hidden files:

```bash
ls -d .[!.]?*
```

- This pattern expands into every filename that begins with a period, does
  not include a second period, contains at least one additional character, and
  may be followed by any other characters.

## Tilde Expansion

When used at the beginning of a word, it expands to the user's home directory.

### Arithmetic Expansion

Arithmetic expansion is performed by using the $((expression)) syntax.

```bash
echo $(( 2 + 2 ))
```

- Arithmetic expansion supports only integers (whole numbers, no deci-
  mals) but can perform quite a number of different operations.

      - Spaces don't matter in arithmetic expressions, and expressions may be
        nested.

```bash
echo $(($((5**2)) * 3))
75
```

- Single parentheses may be used to group multiple subexpressions. With
  this technique, we can rewrite the example above and get the same result
  using a single expansion instead of two:

```bash
echo $(((5**2) * 3))
```

Here's an example using the division and remainder operators.

```bash
echo Five divided by two equals $((5/2))
Five divided by two equals 2
echo with $((5%2)) left over.
with 1 left over.
```

### Tilde Expansion

When the tilde character is used at the beginning of a word, it expands to the
name of the home directory of the named user or, if no user is named, the home
directory of the current user:

```bash
echo ~
/home/jr
```

[[Brace Expansion.md]]
