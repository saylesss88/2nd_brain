---
id: ale
aliases: []
tags: []
---

Plugin 'dense-analysis/ale'

```bash
let g:ale_fixers = {
\   '*': ['remove_trailing_lines', 'trim_whitespace'],
\   'javascript': ['eslint'],
\   'typescript': ['eslint'],
\   'markdown': ['prettier'],
}
```


- Ale supports both linters and fixers. If you don't specify which linters to
  run, all available tools for all supported languages will be run, and you
  might get a correctly formatted file with a bunch of errors. To disable this
  behavior you can tell ALE to run only linters you've explicitly configured:

```bash
let g:ale_linters_explicit = 1
```

- To have ALE run prettier on save:

```bash
let g:ale_fix_on_save = 1
```
