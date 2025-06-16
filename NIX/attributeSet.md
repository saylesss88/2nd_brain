---
id: attributeSet
aliases: []
tags: []
---

# Nix Attribute Set `{ ... }`

```nix Attribute Set
{
  string = "hello";
  integer = 1;
  float = 3.141;
  bool = true;
  null = null;
  list = [ 1 "two" false ];
  attribute-set = {
    a = "hello";
    b = 2;
    c = 2.718;
    d = false;
  };  # comments are supported
}
```

## Recursive Attribute Set `rec { ... }`

```nix
rec {
  one = 1;
  two = one + 1;
  three = two + 1;
}
```

Output:
`{ one = 1; three = 3; two = 2; }`

## `let ... in ...`

`let` expressions allow assigning names to values for repeated use.

```nix
let
  a = 1;
in
a + a
```

Output: `2`

```nix
let
  b = a + 1;
  a = 1;
in
a + b
```

Output: `3`

-   Only expressions within the `let` expression itself can access the newly
    declared names. We say: the bindings have local scope.

## Attribute access

```nix
let
  attrset = { x = 1; };
in
attrset.x
```

Output: `1`

**Access nested attributes**:

```nix
let
  attrset = { a = { b = { c = 1; }; }; };
in
attrset.a.b.c

# Output: `1`
```

## `with ...; ...`

The `with` expression allows us to access attributes without repeatedly
referencing their attribute set.

```nix
let
  a = {
    x = 1;
    y = 2;
    z = 3;
  };
in
with a; [ x y z ]
```

Output: `[ 1 2 3 ]`

`with a; [ x y z] == [ a.x a.y a.z ]`

## `inherit ...`

`inherit` is shorthand for assigning the value of a name from an existing scope
to the same name in a nested scope. It is for convenience to avoid repeating the
same name multiple times.

```nix
let
  x = 1;
  y = 2;
in
{
  inherit x y;
}
```

Output: `{ x = 1; y = 2; }`

`inherit xy; == x = x; y = y;`

-   It's also possible to `inherit` names from specific attribute sets with
    parentheses (`inherit (...) ...`)

```nix
let
  a = { x = 1; y = 2; };
in
{
  inherit (a) x y;
}
```

## String interpolation `${ ... }`

The value of a Nix expression can be inserted into a character string with the
dollar-sign and braces (`${ }`)

```nix
let
  name = "Nix";
in
"hello ${name}"
```

[[fileSystemPaths.md]]
