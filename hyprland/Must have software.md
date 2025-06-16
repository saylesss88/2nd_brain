---
id: Must have software
aliases:
  - Must have software
tags: []
---

# Must have software

**A notification daemon**

Starting method: most likely manual (`exec-once`)

Many apps may freeze without one running.

Examples: `dunst`, `mako`, and `swaync`

**Pipewire**

Starting method: Automatic on systemd, manual otherwise.

Install `pipewire` and `wireplumber` (not `pipewire-media-session`)

**XDG Desktop Portal**  

Starting method: Automatic on systemd, manual otherwise.

XDG Desktop Portal handles a lot of stuff on your desktop, like file pickers,
screensharing, etc.

See the [Desktop Portal](https://github.com/flatpak/xdg-desktop-portal-hyprland)

**Authentication Agent**

Starting method: manual (`exec-once`)

Authentication agents are the things that pop up a window asking you for a
password whenever an app wants to elevate its priveleges.

The recommendation is the KDE one, its `polkit-kde-agent` on Arch.

You can autostart it with exec-once=systemctl --user start plasma-polkit-agent.

On distributions that use a different init system, such as Gentoo, it may be necessary to use exec-once=/usr/lib64/libexec/polkit-kde-authentication-agent-1 instead.

Other possible paths include /usr/lib/polkit-kde-authentication-agent-1, /usr/libexec/polkit-kde-authentication-agent-1, /usr/libexec/kf5/polkit-kde-authentication-agent-1, and /usr/libexec/kf6/polkit-kde-authentication-agent-1.

**Qt Wayland Support**

Starting method: None (just a library)

Install `qt5-waylant` and `qt6-wayland`
