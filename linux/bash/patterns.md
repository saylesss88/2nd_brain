---
id: patterns
aliases:
  - Patterns
tags:
  - bash
  - patterns
---

# Patterns

BASH offers three different kinds of pattern matching.

1. globs

2. extended globs

3. regular expressions

Pattern matching serves two roles in the shell:

- selecting filenames within a directory

- or determining whether a string conforms to a desired format.

_Pattern_: A pattern is a string with a special format designed to match
filenames, or to check, classify or validate data strings.

## Glob Patterns

Globs are basically patterns that can be used to match filenames or other
strings.

Globs are composed of normal characters and metacharacters. Metacharacters are
characters that have special meanings in the shell. These are the metacharacters
that can be used in globs:

- `*` matches any string, including the null string.

- `?` matches any single character.

- `[...]` matches any of the enclosed characters.

Globs are implicitly _anchored_ at both ends. This means that a glob must match
a whole string (filename or data string).

- A glob of `a*` will not match the string `cat`, because it only matches the
  `at`, not the whole string. `ca*` would match `cat`.

An example of using glob patterns to expand to filenames:

```bash
ls
a abc b c
echo *
a abc b c
echo a*
a abc
```

- Bash sees the glob, expands it, by looking in the current directory and
  matching it against all files there. Any filenames that match the glob are
  gathered up and sorted, and then the list of filenames is used in place of the
  glob.

- When a glob is used to match filenames, the `*` and `?` cannot match a slash
  (`/`) character. For example, the glob `*/bin` might match `foo/bin` but cannot
  match `/usr/local/bin`. When globs match patterns, the `/` restriction is
  removed.

In addition to filename expansion, globs may also be used to check whether data
matches a specific format. For example, we might be given a filename, and need
to take different actions depending on its extension:

```bash
$ filename="somefile.jpg"
$ if [[ $filename = *.jpg ]]; then
> echo "$filename is a jpeg"
> fi
somefile.jpg is a jpeg
```

- The `[[` keyword and the `case` keyword both offer the opportunity to check a
  string against a glob -- either regular globs, or extended globs, if the
  latter have been enabled.

## Extended Glob Patterns

The feature is turned off by default, but can be turned on with the `shopt`
command, which is used to toggle shell options:

```bash
shopt -s extglob
```

- `?(list)`: Matches zero or one of the enclosed characters.

- `*(list)`: Matches zero or more of the enclosed characters.

- `+(list)`: Matches one or more of the enclosed characters.

- `@(list)`: Matches one of the enclosed characters.

- `!(list)`: Matches anything except one of the enclosed characters.

The list inside the parentheses is a list of globs or extended globs separated
by the `|` operator.

```bash
$ ls
names.txt  tokyo.jpg  california.bmp
$ echo !(*jpg|*bmp)
names.txt
```

- Out extended glob expands to anything that does not match the `*jpg` or `*bmp`
  pattern.

### Regular Expressions

Regular expressions can only be used for pattern matching, not for filename
matching.

Bash supports the `=~` operator to the `[[` keyword. This operator matches the
string that comes before it against the regex pattern that follows it.

- When the string matches the pattern, `[[` returns with an exit code of `0`
  ("true"). If the string doesn't match the pattern, an exit code of `1` ("false")
  is returned. In case the pattern's syntax is invalid, `[[` will abort the
  operation and return an exit code of `2`.

```bash
$ langRegex='(..)_(..)'
$ if [[ $LANG =~ $langRegex ]]
> then
>     echo "Your country code (ISO 3166-1-alpha-2) is ${BASH_REMATCH[2]}."
>     echo "Your language code (ISO 639-1) is ${BASH_REMATCH[1]}."
> else
>     echo "Your locale was not recognised"
> fi
```

- Regex should always be unquoted. You should protect any special characters by
  excaping them using a backslash.
