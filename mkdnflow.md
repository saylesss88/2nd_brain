# mkdnflow

- `<CR>`: Triggers a wrapper function which will (a) infer your editor mode, and
  then if in normal or visual mode, either follow a link, create a new link from
  the word under the cursor or visual selection, or fold a section (if the
  cursor is on a heading); if in insert mode, it will create a new list item (if
  the cursor is on a list)

- `<Tab>`: Move cursor to the beginnning of the next link if one exists.

- `<C-Space>`: Toggle todo list and cycle options.

- `<leader>nn`: Update numbering for all siblings of the list item of the current
  line.

- `<leader>p`: Create a link, using the content from the system clipboard (e.g.
  a URL) as the source and the word under cursor or visual selection as the link
  text.

- `--`: Create a link from the word under the cursor (in normal mode) or from
  the visual selection (in visual mode).

- `]]`: Next heading.

- `[[`: Previous heading.

- `<BS>`: Open the historically last-active buffer in the current window.

- `<A-CR>`: Destroy the link under the cursor, replacing it with just the text
  from `[...]`.

- `yaa`: Yank a formatted anchor link (if the cursor is on a link).

- `yfa`: Yank a formatted anchor link with the filename included before the
  anchor (if cursor is currently on a line with a heading)
