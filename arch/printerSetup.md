---
title: Printer Setup
date: 2024-11-25
tags: [arch, printer]
---

# Printer Setup

1. Install CUPS and HP related packages:
`sudo pacman -S cups cups-pdf hplip python-pyqt5 python-pyqt5`

2. Enable CUPS:
`sudo systemctl enable cups.service`

3. Add printer to hplip:
`sudo hp-setup -u`
or
`sudo hp-setup -i`

## Troubleshooting

- Ensure `ufw` isn't blocking the printer port:
`sudo ufw allow 631/tcp`

