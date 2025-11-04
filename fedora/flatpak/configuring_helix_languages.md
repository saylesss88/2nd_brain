This is the insecure way of doing things, `flatpak-spawn` bypasses the sandbox.

```toml
[[language]]
name = "markdown"
auto-format = true
language-servers = ["marksman", "markdown-oxide", "harper-ls"]
file-types = ["md", "markdown"]
scope = "source.markdown"
roots = []
indent = {tab-width = 4, unit = "    "}
soft-wrap = {enable = true}

[language.formatter]
command = "flatpak-spawn"
args = [
	"--host",
	"/home/linuxbrew/.linuxbrew/bin/prettier",
	"--parser",
	"markdown",
	"--prose-wrap",
	"always",
]

[language-server.harper-ls]
command = "flatpak-spawn"
args = ["--host", "/home/linuxbrew/.linuxbrew/bin/harper-ls", "--stdio"]

[language-server.harper-ls.config.harper-ls]
userDictPath = ""
workspaceDictPath = ""
fileDictPath = ""
diagnosticSeverity = "hint"
isolateEnglish = false
dialect = "American"
maxFileLength = 120000
ignoredLintsPath = []

[language-server.harper-ls.config.harper-ls.linters]
SpellCheck = true
SpelledNumbers = false
AnA = true
SentenceCapitalization = true
UnclosedQuotes = true
WrongQuotes = false
LongSentences = true
RepeatedWords = true
Spaces = true
Matcher = true
CorrectNumberSUFFIX = true

[language-server.harper-ls.config.harper-ls.codeActions]
ForceStable = false

[language-server.harper-ls.config.harper-ls.markdown]
IgnoreLinkTitle = false

[[language]]
name = "toml"
auto-format = true
language-servers = ["taplo"]

# ────── Formatter ──────
[language.formatter]
command = "flatpak-spawn"
args = [
	"--host",
	"/home/linuxbrew/.linuxbrew/bin/taplo",
	"fmt",
	"--option",
	"indent_string=\t",
	"--option",
	"compact_inline_tables=true",
	"-",                                    # <-- read from stdin, write to stdout
]

# ────── Language server ──────
[language-server.taplo]
command = "flatpak-spawn"
args = ["--host", "/home/linuxbrew/.linuxbrew/bin/taplo", "lsp", "stdio"]

# ────── LSP config (git-workaround) ──────
[language-server.taplo.config]
root_dir = [".git", "*.toml"]
```

## The Secure Way

- [Helix First Run](https://github.com/flathub/com.helix_editor.Helix/blob/master/HELIX_FIRST_RUN.md)

Find available Sdk's:

```bash
flatpak search org.freedesktop.Sdk.Extension
```

Install an extension:

```bash
flatpak install flathub org.freedesktop.Sdk.Extension.rust-stable
flatpak install flathub org.freedesktop.Sdk.Extension.llvm16
```

Enable SDK Extensions:

You need to set `FLATPAK_ENABLE_SDK_EXT` environment variable to a
comma-separated list of extension names:

```bash
FLATPAK_ENABLE_SDK_EXT=rust-stable,llvm16 flatpak run com.helix_editor.Helix
```
