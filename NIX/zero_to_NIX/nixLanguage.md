---
id: nixLanguage
aliases: []
tags: []
---

# The Nix Language

Nix as a programming language can be thought of as a kind of "JSON, but with
functions".

All statements are declarative, meaning that there's no sequential flow of
instructions that makes up a Nix package. Instead, functions are called that
assign values to fields in attribute sets, which in turn may get assigned to
other values.

## How does Nix work?

Nix is a pure, functional, lazy, declarative, and reproducible programming
language.

| Concept      | Description                                                   |
| ------------ | ------------------------------------------------------------- |
| Pure         | Functions don't cause side effects.                           |
| Functional   | Functions can be passed as arguments and returned as results. |
| Lazy         | Not evaluated until needed to complete a computation.         |
| Declarative  | Describing a system outcome.                                  |
| Reproducible | Operations that are performed twice return same results       |

# Syntax Basics

The code below calls a function called `my_function` with the parameters `2` and
`3`, and assigns its output to the `my_value` field:

```nix
{
  my_value = my_function 2 3;
}
```

Functions are defined using this syntax, where `x` and `y` are attributes passed
into the function:

```nix
{
  my_function = x: y: x + y;
}
```

-   The body of the function automatically returns the result of the function.
    Functions are called by spaces between it and its parameters. No commas are
    needed to separate parameters.

The two most common data structures are _attribute sets_ and _lists_. Attribute
sets are key-value pairs, and lists contain different types of values and don't
need to be comma separated.

```nix Recursive attributes
rec {
  number_key = 5;
  list_key = [ number_key true "Hello" ];
}
```

-   The `rec` keyword allows the attribute set to reference itself.
