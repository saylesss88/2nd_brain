---
id: Vim Registers
aliases:
  - Vim Registers
tags: []
---

# Vim Registers

- `:registers` or `:reg` - Display the content of your registers.

- `"<reg>` - This keystroke specifies the register `<reg>` to be read or written.

How do you know when the register `<reg>` is read or written usint the keystroke
`"`? It depends on the keystroke you use afterward. For example:

  - To write the register `a`:
      1. Hit `"a` in NORMAL mode to specify what register you want to write on.
      2. Yank, change, or delete some content (for example by using `dd` in 
      NORMAL mode) to write it to `a`.
  - To read the register `a`:
      1. Hit `"a` in NORMAL mode to specify what register you want to read from.
      2. Use a put keystroke in normal mode (for example `p` or `P`) to spit out 
      the content of the register in your current buffer.

## Types of Registers

1. The unnamed register (`"`) - Contain the last deleted, changed, or yanked 
  content, even if one register was specified.

2. The numbered registers (from `0` to `9`) - Contain the content you've deleted or
  changed.
  1. Each time you delete or change some content, it will be added to `1`
  2. The previous content of the register `1` will be assigned to register `2`
  3. When something is added to the register 1, the content of the register 9
    is lost.

**Writing to a register**

To write the current selection to a register, you can use the `y` command followed
by the register name:

```Vim
"ay
```
This will yank the vilually selected text into the register `a`.

**Reading from a register**

```Vim
"x p
```
Replace `x` with the desired register name. This will paste the contents of 
register `x` at the current cursor position.

- Appending to a register: You can append text to a register usint the `A` command
instead of `y`.

- Deleting to a Register: press `d` followed by the register name.

- Register Manipulation: There are other commands for manipulating registers,
such as `:let`, `:call`, and functions.

