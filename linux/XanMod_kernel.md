---
id: XanMod_kernel
aliases:
  - XanMod_kernel
tags: []
---

# XanMod_kernel

XanMod is a general-purpose Linux kernel distribution with custom settings and
new features. Built to provide a stable, smooth and solid system experience.

Main Features

    Core and Process Scheduling, Load Balancing, Caching, Virtual Memory Manager and CPUFreq Governor optimized for heavy workloads.
    Full multi-core block layer runqueue requests for high I/O throughput.
    ORC Unwinder for kernel stack traces (debuginfo) implementation.
    Real-time Linux kernel (PREEMPT_RT) build available [6.6-rt].
    Third-party patchset available: patches
        Cloudflare's TCP collapse processing for high throughput and low latency [info].
        Google's Multigenerational LRU framework [default].
        Google's BBRv3 TCP congestion control [built-in: tcp_bbr] [default].
        Netfilter nf_tables RFC3489 full-cone NAT support.
        Netfilter FLOWOFFLOAD target to speed up processing of packets.
        NT synchronization primitives emulation driver [as module: ntsync].
        Valve's Steam Deck EC sensors / MFD core and LEDs driver support
        [as module: steamdeck, steamdeck-hwmon, leds-steamdeck] [6.10].
        PCIe ACS Override for bypassing IOMMU groups support.
        Graysky's additional GCC and Clang CPU options.
        Clear Linux patchset [partial].
        Android Binder IPC driver for Waydroid [as module: binder_linux].
    Generic packages for compatibility with any Debian or Ubuntu based distribution.
    GPLv2 license. Can be built for any distribution or purpose.

