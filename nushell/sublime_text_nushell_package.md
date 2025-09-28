---
id: Sublime Text Nushell Package
aliases: [nushell]
---

# Install

```nu
cd ~/.config/sublime-text/Packages/
git clone https://github.com/stevenxxiu/sublime_text_nushell Nushell
```

# Development Setup

To setup the project for development run:

```nu
$ cd Nushell/
$ pre-commit install --hook-type pre-commit --hook-type commit-msg
$ pre-commit run --all-files
```

## To add Nushell syntax highlighting to bat

```nu
bat --generate-config-file
cd ~/.config/bat/
mkdir syntaxes; cd syntaxes
git clone https://github.com/stevenxxiu/sublime_text_nushell
bat cache --build
```
