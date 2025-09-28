---
id: starshipScript
aliases: []
tags: []
---

```bash
1   # If not running interactively, don't do anything
  1 [[ $- != *i* ]] && return
  2
  3 # Load starship prompt if starship is installed
  4 if [ -x /usr/bin/starship ]; then
  5   __main() {
  6     local major="${BASH_VERSINFO[0]}"
  7     local minor="${BASH_VERSINFO[1]}"
  8
  9     if ((major > 4)) || { ((major == 4)) && ((minor >= 1)); }; then
 10       source <("/usr/bin/starship" init bash --print-full-init)
 11     else
 12       source /dev/stdin <<<"$("/usr/bin/starship" init bash --print-full-init)"
 13     fi
 14   }
 15   __main
 16   unset -f __main
 17 fi
```

Replace some more things with better alternatives

```bash
alias cat='bat --style header --style snip --style changes --style header'
[ ! -x /usr/bin/yay ] && [ -x /usr/bin/paru ] && alias yay='paru'
```
