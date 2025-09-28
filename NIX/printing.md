---
id: printing
aliases: []
tags:
    - NixOS printing
---

# Printing

`hp-makeuri` fails to find my network printer.

Solution:

1. Determine the IP address of your printer by printing out the network
   configuration page or use printers touch screen.

2. Run:

```bash
hp-makeuri <ip-address>
hp-makeuri 192.168.0.24  # My printers ip-address
```

-   This will result in a device URI printed to the console:
    `hp:/net/OfficeJet_Pro_6970?ip=192.168.0.24`

1. Copy the URI
2. Open the CUPS UI:
   `http://localhost:631`

Use this command to add the printer entry with the correct device URI:

```bash
lpadmin -p HP_OfficeJet_Pro_6970_F7873C -E -v ipp://192.168.0.24/ipp/print -m everywhere
```

Run `lpstat -v` to verify that the printer is now correctly configured to use
the `ipp://192.168.0.24/ipp/print`
