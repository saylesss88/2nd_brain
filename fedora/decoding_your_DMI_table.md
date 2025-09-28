---
id: decoding_your_DMI_table
aliases:
  - dmidecode – Decoding your system’s DMI table
tags: []
---

# dmidecode – Decoding your system’s DMI table

`dmidecode` is a command-line utility for retrieving detailed information about
your system's hardware.

  -  It reads the DMI (Desktop Management Interface) table, which contains data
  provided by the system’s firmware. This data includes details about the
  system’s BIOS, processor, memory, and other hardware components. Using
  dmidecode, you can gain insights into the hardware configuration without the
  need to be on-site or opening the system case.

## Basic Usage

To get a basic overview of your system's DMI table:

```bash
sudo dmidecode
```
  - This outputs an extensive list of DMI table entries..

To narrow the output, you can use various options, especially via defining a
type with `-t number`:

```bash
man dmidecode
```
  - Where `number` is an integer

Output:
```bash
[... output omitted for readability ...]
The SMBIOS specification defines the following DMI types:
Type   Information
────────────────────────────────────────────
  0   BIOS
  1   System
  2   Baseboard
  3   Chassis
  4   Processor
  5   Memory Controller
  6   Memory Module
  7   Cache
  8   Port Connector
  9   System Slots
  10   On Board Devices

  ...
```
To retrieve BIOS information

```bash
sudo dmidecode -t 0
```

[[Listing PCI devices.md]]


