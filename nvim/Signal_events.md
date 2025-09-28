---
id: Signal_events
aliases:
  - Signal_events
tags: []
---

# Signal_events

Events are signals that trigger specific actions within the editor. They act as
hooks that allow you to execute code or perform tasks based on certain events.

- `BufWritePre`: This event fires before a buffer is written to disk. It's a
  good time to perform actions like:

  - Formatting: You can automatically format the buffer content before saving
    using formatters like `ale` or `conform.nvim`
  - Linting: You can run linters to check for syntax errors or code style
    violations before saving.
  - Pre-commit checks: You can integrate with tools like `husky` to run pre-
    commit checks before saving.

- `BufEnter`: This event fires when you enter a buffer It can be used to:

  - Highlighting: Set specific syntax highlighting for a buffer based on filetype
  - Loading filetype-specific plugins or configurations.
  - Plugin activation: Load plugins relevant tot the current buffer's filetype
  - Autocommands: Run custom commands or configurations associated with the
    previous buffer.

- `BufLeave`: This event fires when you leave a buffer. It can be used to:

  - Cleaning up: Clear temporary buffers or configurations associated with the
    previous buffer.
  - Saving changes: Optionally prompt the user to save changes before leaving

- `VimResized`:

FileType
: This event fires when the filetype of the current buffer is detected. It can be used for:

- Plugin activation: Load plugins specifically designed for the detected filetype.
- Indentation: Set the appropriate indentation style based on the filetype (e.g., spaces for Python, tabs for JavaScript).

- `VimEnter`: Triggered when Neovim is started. It can be used to:

  - Setting global options or keymaps
  - Loading plugins or configurations
  - Initializing variables or other global state

- `VeryLazy`: Not an event: It's a plugin that provides lazy loading
  capabilities for other plugins.
  - Purpose: Optimizes startuptime by delaying plugin loading until necessary.
  - Impact on events: VeryLazy might affect when events like BufEnter or
    VimEnter are triggered for lazy-loaded plugins, as they might not be loaded
    until the event occurs.

Key Differences:

    Timing: VimEnter happens once per Neovim/Vim session, while BufEnter occurs for each new buffer.
    Scope: VimEnter is global, while BufEnter is buffer-specific.
    Lazy Loading: VeryLazy is a tool to manage plugin loading, not an event itself.

When to use Which:

- Use `VimEnter` for actions that need to happen once per session, like
  setting global options or loading core plugins.

- Use `BufEnter` for actions that depend on the specific buffer, like setting
  filetype specific options or loading plugins based on the file content.

- Use `VeryLazy` to optimize plugin loading performance.

Example:

```vim
" Set global options on VimEnter
autocmd VimEnter * set number relativenumber

" Set filetype-specific options on BufEnter
autocmd BufEnter *.py set filetype=python
```

Additional Notes:

    You can combine these events to achieve specific behaviors. For example, you might use VimEnter to load core plugins and BufEnter to load filetype-specific plugins.
    Be aware of the order of events and potential conflicts when using multiple autocommands.

By understanding these differences, you can effectively use VimEnter and BufEnter to create tailored behavior for your Neovim/Vim environment.
