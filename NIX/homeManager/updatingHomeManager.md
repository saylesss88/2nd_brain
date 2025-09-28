---
id: updatingHomeManager
aliases: []
tags:
    - home_manager update
---

# Updating

If you installed Home Manager using the Nix channel method then updating is done
by first updating the channel. You can then switch to the updated Home Manager
environment.

```nix
nix-channel --update

home-manager switch
```

[[nixFlakesWithHomeManager.md]]
