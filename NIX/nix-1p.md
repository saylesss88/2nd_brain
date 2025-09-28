---
id: nix-1p
aliases: []
tags: []
---

# NixLang 1 Pager

Nix is:

-   **purely functional**: It has no concept of sequential steps being executed,
    any dependency between operations is established by depending on _data_ from
    previous operations.

-   Any valid piece of Nix code is an _expression_ that returns a value.

-   Evaluating a Nix expression yields a single data structure, it doesn't execute
    a sequence of operations.

    -   Every Nix file evaluates to a single expression.

-   **lazy**: It will only evaluate expressions when their result is actually
    requested.

-   **purpose-built**: Nix only exists to be the language for Nix, the package
    manager. While people have occasionally used it for other use-cases, it is
    explicitly not a general purpose language.

## Language Constructs

```nix
# numbers
42
1.72394

# strings & paths
"hello"
./some-file.json

# strings support interpolation
"Hello ${name}"

# multi-line strings (common prefix whitespace is dropped)
''
first line
second line
''

# lists (note: no commas!)
[ 1 2 3 ]

# attribute sets (field access with dot syntax)
{ a = 15; b = "something else"; }

# recursive attribute sets (fields can reference each other)
rec { a = 15; b = a * 2; }
```

## `//` (merge) operator

The `//` operator is used pervasively in Nix code.

It merges the left and right attribute sets given to it:

```nix
{ a = 1; } // { b = 2; }

# yields { a = 1; b = 2; }
```

Values from the right side take precedence:

```nix
{ a = "left"; } // { a = "right"; }

# yields { a = "right"; }
```

## Variable bindings

Bindings in Nix are introduced locally via `let` expressions, which make some
variables available within a given scope:

```nix
let
  a = 15;
  b = 2;
in a * b

# yields 30
```

-   Variables are immutable. This means that after defining what `a` or `b` are,
    you can not modify their value in the scope in which they are available.
