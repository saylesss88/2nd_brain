---
id: evaluationCaching
aliases: []
tags: []
---

# Evaluation Caching

Nix evaluation is slow because in order to get any information about
modules,..., Nix first needs to evaluate a substantial Nix
program. This involves parsing potentially thousands of `.nix` files and running
a Turing-complete language.
