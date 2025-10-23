## Install zoxide and atuin

### Zoxide for Nushell

Add this to the end of `env.nu`:

```nu
zoxide init nushell | save -f ~/.zoxide.nu
```

And add this to the end of your `config.nu`:

```nu
source ~/.zoxide.nu
```

### Atuin for Nushell

```bash
mkdir ~/.local/share/atuin/
atuin init nu | save ~/.local/share/atuin/init.nu
```

Add to `config.nu`:

```nu
source ~/.local/share/atuin/init.nu
```
