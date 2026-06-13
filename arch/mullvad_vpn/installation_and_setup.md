1. Install `mullvad-vpn`:

```sh
sudo pacman -S mullvad-vpn
```

2. Enable the daemon:

```sh
sudo systemctl enable --now mullvad-daemon
mullvad account login <your-account-number>
mullvad connect
```

3. Launch the app, enter your account number and choose a location.
