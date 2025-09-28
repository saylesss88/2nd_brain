---
id: kill-9
aliases: []
tags: []
---

The `kill -9` command sends a SIGKILL signal to a service, shutting it down
immediately. 

- An unresponsive program ignores a kill command, but it shuts down whenever a
  `kill -9` command is issued.

- Use this comand with caution since it bypasses the standard shutdown routine,
  and any unsaved data will be lost.

## xkill command

The `xkill` command is a special command that closes a given server's connection
to clients.

```bash
xkill [resource]
```


