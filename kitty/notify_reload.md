---
id: notify_reload
aliases: []
tags: []
---

1. Create a script `notify_reload.sh` in your `~/.config/kitty/` directory:

```bash
#!/bin/bash
notify-send "Kitty" "Configuration Reloaded!!!!"
```

2. Make the script executable:

```bash
chmod +x ~/.config/kitty/notify_reload.sh
```
