---
id: inxi
aliases: []
tags: []
---

Get detailed information about your system.

```shell
inxi -Fxz
```

Output:
```bash
System:
  Kernel: 6.10.3-zen1-2-zen arch: x86_64 bits: 64 compiler: gcc v: 14.2.1
  Desktop: Hyprland v: 0.41.2 Distro: Garuda base: Arch Linux
Machine:
  Type: Laptop System: Dell product: XPS 15 9550 v: N/A
    serial: <superuser required>
  Mobo: Dell model: 0N7TVV v: A00 serial: <superuser required> UEFI: Dell
    v: 1.14.0 date: 02/13/2020
Battery:
  ID-1: BAT0 charge: 46.0 Wh (100.0%) condition: 46.0/56.0 Wh (82.2%)
    volts: 12.5 min: 11.4 model: Samsung SDI DELL M7R965B status: full
CPU:
  Info: quad core model: Intel Core i7-6700HQ bits: 64 type: MT MCP
    arch: Skylake-S rev: 3 cache: L1: 256 KiB L2: 1024 KiB L3: 6 MiB
  Speed (MHz): avg: 1147 high: 2100 min/max: 800/3500 cores: 1: 2100 2: 800
    3: 1179 4: 800 5: 1501 6: 800 7: 1200 8: 800 bogomips: 41599
  Flags: avx avx2 ht lm nx pae sse sse2 sse3 sse4_1 sse4_2 ssse3 vmx
Graphics:
  Device-1: Intel HD Graphics 530 vendor: Dell XPS 15 9550 driver: i915
    v: kernel arch: Gen-9 bus-ID: 00:02.0
  Device-2: NVIDIA GM107M [GeForce GTX 960M] vendor: Dell XPS 15 9550
    driver: nvidia v: 555.58.02 arch: Maxwell bus-ID: 01:00.0
  Device-3: Microdia Integrated_Webcam_HD driver: uvcvideo type: USB
    bus-ID: 1-12:5
  Display: wayland server: X.org v: 1.21.1.13 with: Xwayland v: 24.1.2
    compositor: Hyprland v: 0.41.2 driver: X: loaded: modesetting,nvidia
    unloaded: nouveau dri: iris gpu: i915 resolution: 1: 1920x1080~60Hz
    2: 1920x1080~60Hz
  API: Vulkan v: 1.3.279 drivers: intel,nvidia,llvmpipe
    surfaces: xcb,xlib,wayland devices: 3
  API: EGL Message: EGL data requires eglinfo. Check --recommends.
Audio:
  Device-1: Intel 100 Series/C230 Series Family HD Audio
    vendor: Dell XPS 15 9550 driver: snd_soc_avs v: kernel bus-ID: 00:1f.3
  API: ALSA v: k6.10.3-zen1-2-zen status: kernel-api
  Server-1: sndiod v: N/A status: off
  Server-2: PipeWire v: 1.2.2 status: active
Network:
  Device-1: Broadcom BCM43602 802.11ac Wireless LAN SoC vendor: Dell
    driver: brcmfmac v: kernel bus-ID: 02:00.0
  IF: wlan0 state: up mac: <filter>
Bluetooth:
  Device-1: Broadcom BCM20703A1 Bluetooth 4.1 + LE driver: btusb v: 0.8
    type: USB bus-ID: 1-4:4
  Report: btmgmt ID: hci0 rfk-id: 0 state: up address: <filter> bt-v: 4.1
    lmp-v: 7
Drives:
  Local Storage: total: 596.45 GiB used: 39.32 GiB (6.6%)
  ID-1: /dev/nvme0n1 vendor: SanDisk model: THNSN5512GPUK NVMe TOSHIBA 512GB
    size: 476.94 GiB temp: 43.9 C
  ID-2: /dev/sda vendor: Samsung model: Type-C size: 119.51 GiB type: USB
Partition:
  ID-1: / size: 459.63 GiB used: 39.32 GiB (8.6%) fs: btrfs
    dev: /dev/nvme0n1p2
  ID-2: /boot/efi size: 299.4 MiB used: 584 KiB (0.2%) fs: vfat
    dev: /dev/nvme0n1p1
  ID-3: /home size: 459.63 GiB used: 39.32 GiB (8.6%) fs: btrfs
    dev: /dev/nvme0n1p2
  ID-4: /var/log size: 459.63 GiB used: 39.32 GiB (8.6%) fs: btrfs
    dev: /dev/nvme0n1p2
  ID-5: /var/tmp size: 459.63 GiB used: 39.32 GiB (8.6%) fs: btrfs
    dev: /dev/nvme0n1p2
Swap:
  ID-1: swap-1 type: zram size: 15.47 GiB used: 0 KiB (0.0%) dev: /dev/zram0
  ID-2: swap-2 type: partition size: 17.01 GiB used: 0 KiB (0.0%)
    dev: /dev/nvme0n1p3
Sensors:
  System Temperatures: cpu: 50.0 C pch: 67.0 C mobo: 48.0 C
  Fan Speeds (rpm): cpu: 2530 fan-2: 2528
Info:
  Memory: total: 16 GiB available: 15.47 GiB used: 3.2 GiB (20.7%)
  Processes: 265 Uptime: 14m Init: systemd
  Packages: 1335 Compilers: gcc: 14.2.1 Shell: fish v: 3.7.1 inxi: 3.3.35
```
