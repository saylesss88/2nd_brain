# eza install

```bash
sudo dnf install eza
```
aliases for .zshrc
```bash
ld=’eza -lD’
lf=’eza -lf --color=always | grep -v /’
lh=’eza -dl .* --group-directories-first’
ll=’eza -al --group-directories-first’
ls=’eza -alf --color=always --sort=size | grep -v /’
lt=’eza -al --sort=modified’
```
What they do...

```bash
ld — lists only directories (no files)
lf — lists only files (no directories)
lh — lists only hidden files (no directories)
ll — lists everything with directories first
ls — lists only files sorted by size
lt — lists everything sorted by time updated
```
