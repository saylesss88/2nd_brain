---
id: shada_file
aliases:
  - Shada File
tags: []
---

# Shada File

The `nvim-shada` file is a Neovim session file that stores information about
your Neovim environment, such as:

- **Open buffers**: The names and contents of the buffers you were editing.

- **Window layout: The arrangment of windows on the screen.

- **Cursor position**: The position of the cursor in the buffer.

- **Marks**: The marks you have set.

- **Registers**: The contents of the registers.

- **Mappings**: The mappings you have set.

When you start Nvim, it will look for the `nvim-shada` file in your
configuration directory and load the saved stat if it exists.

This allows you to quickly resume your Neovim session, including all your open
buffers and their contents.

**To enable or disable the `nvim-shada` file:

1. Open your nvim config (init.vim or init.lua)

2. Set the `shada` option: Use the `set` command to set the `shada` option to
   your desired value. For example:

   ```vim set shada! filename=<path to nvim-shada file

   ```

  Replace <path/to/nvim-shada> with the actual path to the nvim-shada file.
  shada! filename=<path/to/nvim-shada>: Enables the nvim-shada file and specifies the path to the file.
  set noshada: Disables the nvim-shada file.

Additional options for the shada option:

 shadafile=<path/to/nvim-shada>: Specifies the path to the nvim-shada file.  shadafiledir=<directory>: Specifies a directory where the nvim-shada file will be   created.
 shadaformat=<format>: Specifies the format of the nvim-shada file.

By using the nvim-shada file, you can save time and improve your workflow by
quickly resuming your previous Neovim sessions.
