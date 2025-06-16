---
id: indentedStrings
aliases:
    - indentedStrings
tags: []
---

# indentedStrings

Also known as "multi-line-strings".

```nix
''
multi
line
string
''
```

Value:
`"multi\nline\nstring\n"`

Equal parts of prepended white space are trimmed from the result.

```nix
''
  one
    two
      three
''
```

Value: `"one\n two\n  three\n"`

[[nixFunctions.md]]
