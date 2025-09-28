---
id: nix_Repl
aliases: []
tags: []
---

# Nix Repl

List available commands with `:?`:

```nix
nix repl
Nix 2.24.11
Type :? for help.
nix-repl> :?
The following commands are available:

  <expr>                       Evaluate and print expression
  <x> = <expr>                 Bind expression to variable
  :a, :add <expr>              Add attributes from resulting set to scope
  :b <expr>                    Build a derivation
  :bl <expr>                   Build a derivation, creating GC roots in the
                               working directory
  :e, :edit <expr>             Open package or function in $EDITOR
  :i <expr>                    Build derivation, then install result into
                               current profile
  :l, :load <path>             Load Nix expression and add it to scope
  :lf, :load-flake <ref>       Load Nix flake and add it to scope
  :p, :print <expr>            Evaluate and print expression recursively
                               Strings are printed directly, without escaping.
  :q, :quit                    Exit nix-repl
  :r, :reload                  Reload all files
  :sh <expr>                   Build dependencies of derivation, then start
                               nix-shell
  :t <expr>                    Describe result of evaluation
  :u <expr>                    Build derivation, then start nix-shell
  :doc <expr>                  Show documentation of a builtin function
  :log <expr>                  Show logs for a derivation
  :te, :trace-enable [bool]    Enable, disable or toggle showing traces for
                               errors
  :?, :help                    Brings up this help menu
```

## Load directly Nix expressions

You can quickly evaluate a random Nix expression:

```nix
nix repl --expr '{a = { b = 3; c = 4; }; }'

Welcome to Nix 2.11.0. Type :? for help.

Loading installable ''...
Added 1 variables.
nix-repl> a
{ b = 3; c = 4; }
```

-   The `--expr` flag is helpful to prime directly the Nix REPL with valuable data
    or values.

## Load Flakes

We can use the `--expr` flag to load a random Nix Flake directly:

```nix
nix repl --expr 'builtins.getFlake "github:nix-community/ethereum.nix"'
```

Also, you can load a flake directly inside the REPL with `:load-flake` or `:lf`:

```nix
nix repl

nix-repl> :lf github:nix-community/home-manager
```
