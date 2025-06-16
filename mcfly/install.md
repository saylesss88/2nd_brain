---
id: install
aliases: []
tags: []
---

```bash
 curl -LSfs https://raw.githubusercontent.com/cantino/mcfly/master/ci/install.sh | sh -s -- --git cantino/mcfly
```

For Zsh shell, add this to your `.zshrc`:

```bash
eval "$(mcfly init zsh)"
```

