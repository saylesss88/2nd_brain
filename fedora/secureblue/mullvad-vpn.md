# Mullvad VPN

```bash
sudo systemctl enable mullvad-daemon
mullvad auto-connect set on
sudo systemctl enable mullvad-early-boot-blocking
mullvad lockdown-mode set on

mullvad auto-connect get
mullvad lockdown-mode get
```
