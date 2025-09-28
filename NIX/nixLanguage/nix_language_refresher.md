---
date: 12-15-24
aliases: nix language
title: nix language refresher
---

# Nix Language Refresher

The widely used data type in Nix is an attribute set: a data type for storing
key-value pairs.

```nix
{
  hello = "world";
  foo = "bar";
}
```

The set above is equivalent to this JSON object:

```json
{
  "hello": "world",
  "foo": "bar"
}
```

`hello` and `foo` are "attributes" or "attribute names"; `"world"` and `"bar"` are
"attribute values"

To get an attribute value from an attribute set, use `.` For example:

```nix
let
  my_attrset = { foo = "bar"; };
in my_attrset.foo
```

(`let ... in` is a way to create bindings; the syntax inside is identical to
that of an attribute set)

You can also abbreviate your attribute set by setting specific attributes with
`.` instead of defining the entire set:

```nix
{
  foo.bar = "baz";
}
```

Is equivalent to:

```nix
{
  foo = { bar = "baz"; };
}
```

Functions support pattern matching on attribute sets. For example, this
function:

```nix
{ a, b}: a + b
```

When called with `{ a = 10; b = 20; }` will return 30.

Function application is done ML style:

```nix
let
  f = { a, b}: a + b;
in f { a = 10; b = 20; }
```

- The function itself comes first. Then there is a whitespace-separated list of
  arguments.

If you want to have a function of multiple arguments, use currying:

```nix
let
  f = a: b: a + b;
in f 10 20
```

In this example, `f 10` evaluates to `b: 10 + b`, and then `f 10 20` evaluates
to `30`.

[[basicFlakeStructure.md]]
