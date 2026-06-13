# pipewire

```bash
sudo pacman -S pipewire pipewire-audio pipewire-pulse pipewire-alsa wireplumber
```

```bash
systemctl --user enable --now pipewire.service wireplumber.service pipewire-pulse.service

systemctl --user status pipewire wireplumber
```

## Enabling sound out of both monitors

```bash
pactl list sinks short
```

Create a `~/.config/pipewire/pipewire-pulse.conf.d/combine-sink.conf`:

```text
context.exec = [
  { path = "pactl"
    args = "load-module module-combine-sink slaves=alsa_output.pci-0000_05_00.1.HiFi__HDMI2__sink,alsa_output.pci-0000_05_00.1.HiFi__HDMI1__sink sink_name=combined"
  }
]
```

Open `pavucontrol` and select the combined profile.
