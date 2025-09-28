---
id: GNU Stow
aliases:
  - GNU Stow
tags: []
---
# GNU Stow

GNU Stow is a symlink farm manager, which means it helps you store all you
store all your config files in one common location.

- So when you "stow" a file, it will create a symlink between your file and an
identical file in your home directory.

But remember that it will follow the same folder structure as the file. So,
if you have a file like:

```bash
./dotfiles/randomfolder/.zshrc
```

Then, stow will symlink to the directory:

```bash
~/randomfolder/.zshrc
```

The main lesson here is that you should organize your dotfiles in the same
way you would organize them in your home folder.

```bash
./dotfiles
├── .config
└── nvim/
└── init.lua
```

## Using GNU Stow

I would suggest creating a folder in your home directory called dotfiles.

```bash
mkdir ~/.dotfiles
```

This is where all our common configurations will reside. As an example, we would create a file named random.txt.

```bash
cd ~/.dotfiles
touch random.txt
```

Finally, you just have to run `stow .` to symlink my files to my parent directory.

If all went well, you should find a `random.txt` file in your home directory.
