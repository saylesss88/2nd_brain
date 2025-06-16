---
id: readingInParameters
aliases:
  - readingInParameters
tags: []
---

# readingInParameters

Using the `read` command, you can prompt the user for information and store that
info to use later in your script.

```bash
#!/bin/bash
read -p "Type in an adjective, noun and verb (past tense): " adj1 noun1 verb1
echo "He sighed and $verb1 to the elixir. Then he ate the $adj1 $noun1."
```

In this script, after the script prompts for an adjective, noun, and verb, the user is
expected to enter words that are then assigned to the adj1, noun1, and verb1 vari-
ables. Those three variables are then included in a silly sentence, which is displayed on the
screen. If the script were called sillyscript, here’s an example of how it might run:

```bash
$ chmod 755 /home/chris/bin/sillyscript
$ sillyscript
Type in an adjective, noun and verb (past tense): hairy football danced
He sighed and danced to the elixir. Then he ate the hairy football.
```

[[parameterExpansion.md]]
