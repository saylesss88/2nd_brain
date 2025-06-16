---
id: dd and Devices
aliases:
  - dd and Devices
tags: []
---

# dd and Devices

The program `dd` is extremely useful when you are working with block and
character devices.

  - Its sole function is to read from an input file or stream and write to an
     output file or stream.

  - One particularly useful `dd` feature is that you can process a chunk of data
    in the middle of a file, ignoring what comes before or after.

> [!warning] `dd` is very powerful, it's very easy to corrupt files and data on
> devices by making a careless mistake. It helps to write the output to a new
> file if you're not sure what it will do.

`dd` copies data in blocks of a fixed size. Here’s how to use dd with a char-
acter device, utilizing a few common options:
```bash
$ dd if=/dev/zero of=new_file bs=1024 count=1
```

- This copies a single 1024-byte block of data from the device `/dev/zero` to
  new_file.

These are the important `dd` options:

    - if=file  : The input file. Default is stdin.

    - of=file  : The output file. Default is stdout.

    - bs=size  : The block size.

    - ibs=size, obs=size The input and output block sizes. If you can use the
      same block size for both input and output, use the bs option; if not, use
      ibs and obs for input and output, respectively.

    - count=num The total number of blocks to copy. When working with a huge
      file—or with a device that supplies an endless stream of data, such as
      /dev/zero—you want dd to stop at a fixed point; otherwise, you could waste
      a lot of disk space, CPU time, or both. Use count with the skip parameter
      to copy a small piece from a large file or device.
   
    - skip=num Skip past the first num blocks in the input file or stream, and
     do not copy them to the output.

  [[Hard Disks.md]]
