---
id: bashHeredoc
aliases:
  - Bash heredoc
tags: []
---

# Bash heredoc

In Bash and other shells like Zsh, a Here document (Heredoc) is a type of
redirection that allows you to pass multiple lines of text as input to a
command.

The syntax of writing a HereDoc:

```bash
[COMMAND] <<[-] 'DELIMITER'
  HERE-DOCUMENT
DELIMITER
```

- The first line starts with an optional command followed by the special
  redirection operator << and the delimiting identifier

- You can use any string as a delimiting identifier, most common is EOF or END.

- If the delimiting identifier is unquoted, the shell will substitute all
  variables, commands and special characters before passing the here-document
  lines to the command.

- Appending a minus sign to the redirection operator `<<-`, will cause all
  leading tab characters to be ignored. This allows you to use indentation when
  writing a HereDoc in shell scripts. Leading whitespace characters are not
  allowed, only tab.

Basic Example:

```bash
cat << EOF
The current working directory is: $PWD
You are logged in as: $(whoami)
EOF
```

Output is what you would expect. But if you add quotes the behavior changes
completely:

```bash
cat <<- "EOF"
The current working directory is: $PWD
You are logged in as: $(whoami)
EOF
```

Output:

```bash
The current working directory is: $PWD
You are logged in as: $(whoami)
```

If you are using a heredoc inside a statement or loop, use the <<- redirection
operation that allows you to indent your code.

```bash
if true; then
    cat <<- EOF
    Line with a leading tab.
    EOF
fi
```

```bash
# Output:
Line with a leading tab.
```

Instead of displaying the output on the screen you can redirect it to a file
using the `>`, `>>` operators:

```bash
cat << EOF > file.txt
The current working directory is: $PWD
You are logged in as: $(whoami)
EOF
```

- If the `file.txt` doesn't exist it will be created. When using `>` the file
  will be overwritten, while `>>` will append to the file.

You can also use pipes and other commands:

```bash
cat <<'EOF' | sed 's/l/e/g'
Hello
World
EOF
```

```bash
# Output:
Heeeo
Wored
```

To write the piped data to a file:

```bash
cat <<'EOF' | sed 's/l/e/g' > file.txt
Hello
World
EOF
```
