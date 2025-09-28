---
id: stylua
aliases:
  - Stylua Usage
tags: []
---

# Stylua Usage

Once installed, pass the files to format to the CLI:

```lua
stylua src/ foo.lua bar.lua
```

This command will format the `foo.lua` and `bar.lua` file, and search down the
`src` directory to format any files within it. Stylua can also read from stdin,
by using `-` as the file name.

## Glob Filtering

By default, when searching through a directory, StyLua looks for all files
matching the glob `**/*.lua` (or `**/*.luau` when `luau` is enabled) to format.
You can also specify an explicit glob pattern to match against when searching:

```lua
stylua --glob '**/*.luau' -- src # format all files in src matching **/*.luau
stylua -g '*.lua' -g '!*.spec.lua' -- . # format all Lua files except test files ending with `.spec.lua`
```

> [!note]
> If you are using the glob argument, it can take in multiple strings, so `--`
> is required to break between the glob pattern and the files to format.
