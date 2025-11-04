[---](2024-09-09_---.md)-- id: StowNvim aliases:

- StowNvim tags: []

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

## Keyd

> Didn't work

```bash
cd ~/.dotfiles
mkdir -p etc/keyd
sudo mv /etc/keyd ~/.dotfiles/etc/
```

Now, with the files in place, you can use stow to create the symlinks. You must
use the --target option and specify the root directory (/) as the target. The
command will look for the etc/keyd directory inside your ~/.dotfiles and create
a symlink from ~/.dotfiles/etc/keyd to /etc/keyd

```bash
sudo stow -t / etc
```

## Breakdown

- `sudo mv /etc/keyd ~/.dotfiles/etc/`: This moves the `/etc/keyd` dir and it's
  contents to `~/.dotfiles/etc`.

- `sudo stow -t / etc`: `-t /` specifies the **target directory** as the root of
  the filesystem.Instead of the default home directory, Stow will now place its
  symlinks starting from `/`.

- `etc`: This is the **package name** orthe directory inside `~/.dotfiles` that
  we want to symlink. Stow will look for this directory (`~/.dotfiles/etc/`) and
  then, based on the `-t /` flag, create symlinks for its contents inside the
  `/` directory.

After running the command, you will have a symlink at `/etc/keyd` that points to
`~/.dotfiles/etc/keyd`. This allows you to manage the keyd configuration files
from within your dotfiles repository.
