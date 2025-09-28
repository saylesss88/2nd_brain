```bash
sudo pacman -S firejail
# or
paru -S firejail-git
```

## Usage

```bash
firejail firefox
```

**Use by default**

To use Firejail by default for all apps which have profiles:

```bash
sudo firecfg
```

```bash
sudo firejail --noprofile firefox
firejail --noprofile --whitelist=~/.mozilla
firejail --whitelist=/tmp/.X11-unix --whitelist=/dev/null
firejail --whitelist="/home/username/My Virtual Machines"
firejail --whitelist=/home/username/My\ Virtual\ Machines
firejail --whitelist=~/work* --whitelist=/var/backups*
```
