---
id: Atuin_Config
aliases:
  - Atuin_Config
tags: []
---

# Atuin_Config

Atuin maintains two config files, stored in `~/.config/atuin/`.
We store data in `~/.local/share/atuin`

- The full path to the config file would be `~/.config/atuin/config.toml`

- The config location can be overridden with ATUIN_CONFIG_DIR

**`db_path`**
Default: `~/.local/share/atuin/history.db`

The path to the Atuin SQLite database.

```toml
db_path = "~/.history.db"
```

## History list

`atuin history list --cwd`
