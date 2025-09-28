---
id: AMDGPU
aliases: []
tags: []
---

```bash
sudo pacman -Syu mesa vulkan-radeon lib32-mesa lib32-vulkan-radeon linux-firmware amd-ucode
```

`/etc/mkinitcpio.conf`:

```bash
HOOKS = (... microcode ...)
```

```bash
cat /proc/cpuinfo | grep "model name"
model name      : AMD Ryzen 7 5700U with Radeon Graphics
model name      : AMD Ryzen 7 5700U with Radeon Graphics
model name      : AMD Ryzen 7 5700U with Radeon Graphics
model name      : AMD Ryzen 7 5700U with Radeon Graphics
model name      : AMD Ryzen 7 5700U with Radeon Graphics
model name      : AMD Ryzen 7 5700U with Radeon Graphics
model name      : AMD Ryzen 7 5700U with Radeon Graphics
model name      : AMD Ryzen 7 5700U with Radeon Graphics
model name      : AMD Ryzen 7 5700U with Radeon Graphics
model name      : AMD Ryzen 7 5700U with Radeon Graphics
model name      : AMD Ryzen 7 5700U with Radeon Graphics
model name      : AMD Ryzen 7 5700U with Radeon Graphics
model name      : AMD Ryzen 7 5700U with Radeon Graphics
model name      : AMD Ryzen 7 5700U with Radeon Graphics
model name      : AMD Ryzen 7 5700U with Radeon Graphics
model name      : AMD Ryzen 7 5700U with Radeon Graphics
```
