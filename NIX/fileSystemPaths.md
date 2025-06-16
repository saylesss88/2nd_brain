---
id: fileSystemPaths
aliases:
    - fileSystemPaths
tags: []
---

# fileSystemPaths

Absolute paths always start with a slash (`/`).

`/absolute/path`

Paths are relative when they contain at least one slash but do not start with
one. They evaluate to the path relative to the file containing the expression.

Relative:
`./relative`
`# value: /current/directory/relative`

`./.` # Current directory
`# value: /current/directory`

`../.`
`# value: /current`

## Lookup paths

Also known as "angle bracket syntax"

```nix
<nixpkgs>
```

```nix
/nix/var/nix/profiles/per-user/root/channels/nixpkgs
```

-   The value of a `lookup path` is a file system path that depends on the value
    of the `builtins.nixPath`

-   In practice, `<nixpkgs>` points to the file system path of some revision of
    `Nixpkgs`.

```nix
<nixpkgs/lib>
```

Value:
`/nix/var/nix/profiles/per-user/root/channels/nixpkgs/lib`

[[indentedStrings.md]]
