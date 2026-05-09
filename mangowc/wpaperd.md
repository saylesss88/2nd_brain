# Wpaperd on mangowm

Clone wpaperd and run:

```bash
cargo install --path cli
cargo install --path daemon
```

create `/etc/systemd/user/wpaperd.service`:

```text
[Unit]
Description=wpaperd wallpaper daemon
PartOf=graphical-session.target
After=graphical-session.target

[Service]
Type=simple
ExecStart=/home/jr/.cargo/bin/wpaperd
Restart=on-failure
RestartSec=2

[Install]
WantedBy=graphical-session.target
```

Then enable and start the service:

```bash
systemctl --user daemon-reload
systemctl --user enable --now wpaperd.service
```

If it shows inactive/dead, add this to `autostart.sh`:

```sh
systemctl --user start wpaperd.service &
```
