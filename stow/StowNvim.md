[---](2024-09-09_---.md)--
id: StowNvim
aliases:

- StowNvim
  tags: []

---

# StowNvim

```bash
mkdir ~/.dotfiles
```

```bash
cd ~/.dotfiles
mkdir -p nvim/.config/nvim
mv ~/.config/nvim/ nvim/.config/
```

Stow nvim:

```bash
stow nvim
```

## Stow ZSH

```bash
mkdir -p ~/.dotfiles/zsh
mv ~/.zshrc ~/.dotfiles/zsh/
```

```bash
cd ~/.dotfiles
stow zsh
```

**Verify the Symlink**:

```bash
ls -l ~/.zshrc
```
