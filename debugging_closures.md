---
id: debugging_closures
aliases: []
tags: []
---

# Debugging Closures

```bash
cd ~/src/nixpkgs
rg 'disallowedReferences ='
```

`disallowedReferences` is a way to tell Nix, don't allow this package in the
result of the derivation.
