---
id: archwikiConnman
aliases: []
tags: []
---

Install the `connman` package.`wpa_supplicant`, `bluez`, and `openvpn` are
optional dependencies required for WiFi, Bluetooth, and VPN functionality.

- Before enabling `connman.service`, ensure any existing network configuration
  is disabled.

- Connman comes with the `connmanctl` command-line utility.

## Front ends

- `cmst` - Qt GUI for ConnMan. (AUR)

- `connman-gtk` - GTK GUI for ConnMan. (AUR) discontinued.

- `connman-ui-git` - GTK3 Web interface for ConnMan. (AUR)

### Usage

To check if WiFi is enabled you can run `connmanctl technologies` and check for
the line that says `Powered: True/False`.

- To power the WiFi on you can run `connmanctl enable wifi` or `connmanctl
  disable wifi`.

- Other ways to enable WiFi could include using the `Fn` key or running `ip link
  set interface up`.

#### Connecting to an open access point

To scan the network `connmanctl` accepts simple names called technologies. To
scan for nearby Wi-Fi networks:

```bash
connmanctl scan wifi
```
To list the available networks found after a scan run:

```bash
connmanctl services
```

Example output:

```bash
*AO MyNetwork               wifi_dc85de828967_68756773616d_managed_psk
    OtherNET                wifi_dc85de828967_38303944616e69656c73_managed_psk
    AnotherOne              wifi_dc85de828967_3257495245363836_managed_wep
    FourthNetwork           wifi_dc85de828967_4d7572706879_managed_wep
    AnOpenNetwork           wifi_dc85de828967_4d6568657272696e_managed_none

```

To connect to a network run:

```bash
connmanctl connect wifi_dc85de828967_4d6568657272696e_managed_none
```

Connecting to a protected access point

For protected access points you will need to provide some information to the ConnMan daemon, at the very least a password or a passphrase.

The commands in this section show how to run connmanctl in interactive mode, it is required for running the agent command. To start interactive mode simply type:

$ connmanctl

You then proceed almost as above, first scan for any Wi-Fi technologies:

connmanctl> scan wifi

To list services:

connmanctl> services

Now you need to register the agent to handle user requests. The command is:

connmanctl> agent on

You now need to connect to one of the protected services. To do this easily, just use tab completion for the wifi_ service. If you were connecting to OtherNET in the example above you would type:

connmanctl> connect wifi_dc85de828967_38303944616e69656c73_managed_psk

The agent will then ask you to provide any information the daemon needs to complete the connection. The information requested will vary depending on the type of network you are connecting to. The agent will also print additional data about the information it needs as shown in the example below.

Agent RequestInput wifi_dc85de828967_38303944616e69656c73_managed_psk
  Passphrase = [ Type=psk, Requirement=mandatory ]
  Passphrase?

Provide the information requested, in this example the passphrase, and then type:

connmanctl> quit

If the information you provided is correct you should now be connected to the protected access point.

[[Using iwd instead of wpa supplicant.md]]
