---
id: nix-by-example
aliases: []
tags: []
---

# nix by example

```nix
nix-instantiate --eval --expr '"Hello world"'
"Hello world"
```

- Here, nix took the expression "Hello world", evaluated it, and printed out the
  value. This means "Hello world" is a valid Nix expression.


