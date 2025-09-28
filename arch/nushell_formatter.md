## Topiary-Nushell

```bash
cargo install topiary-cli
```

Set the `$env.XDG_CONFIG_HOME` variable in your `~/.config/nushell/config.nu`:

```nu
$env.XDG_CONFIG_HOME = ($env.HOME | path join ".config")
```

Clone the topiary repo:

```bash
# e.g. to `$env.XDG_CONFIG_HOME/topiary`
git clone https://github.com/blindFS/topiary-nushell ($env.XDG_CONFIG_HOME | path join topiary)
```

Add the following environment variables to `~/.config/nushell/config.nu`:

```nu
# Set environment variables according to the path of the clone
$env.TOPIARY_CONFIG_FILE = ($env.XDG_CONFIG_HOME | path join topiary languages.ncl)
$env.TOPIARY_LANGUAGE_DIR = ($env.XDG_CONFIG_HOME | path join topiary languages)
```

## Setting up Helix to use it

```toml
# languages.toml
[[language]]
name = "nu"
auto-format = true
formatter = {command = "topiary", args = ["format", "--language", "nu"]}
```
