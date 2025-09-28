## Helix Spellchecking

Install harper-ls:

```bash
sudo pacman -S harper
```

Or with cargo:

```bash
cargo install harper-ls --locked
```

Add the following to your `~/.config/helix/languages.toml`:

```toml
[[language]]
name = "markdown"
file-types = ["md", "markdown"]
language-servers = ["harper-ls"]
scope = "source.markdown"
auto-format = true
formatter = {command = "prettier", args = [
	"--parser",
	"markdown",
	"--prose-wrap",
	"always",
]}
soft-wrap = {enable = true}

[language-server.harper]
command = "harper"
args = ["lsp"]

[[language]]
name = "text"
file-types = ["txt"]
scope = "source.text"
language-servers = ["harper"]
```
