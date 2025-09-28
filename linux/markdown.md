## Header 1

## Header 2

The following requires `:filetype plugin on`.``

- `:HeaderDecrease`:
Decrease level of all headers in buffer:`h2` to `h1`, `h3` to `h2`, etc.

- `:HeaderIncrease`:

- `:SetexToAtx`:
Convert all Setex style headers in buffer to Atx style headers.

If a range is given, e.g. hit `:` from visual mode, only operate on the
range.

- `:TableFormat`: Format the table under the cursor.(requires Tabular)

- `:Toc`: create a quickfix vertical window navigable table of contents with
the headers.

```md
header 1|header 2
--|--
data 1|data 2```
