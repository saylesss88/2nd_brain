## Theme

On mdbook/mdbook-toc/handlebars updates issues may arrise. Recently my
gh-actions started failing with an index error. I was able to figure out that it
was talking about the `theme/index.hbs`, I tried fixing the error but should've
just run:

```bash
mdbook init --theme
```

This does overwrite your `book.toml` so copy it first.
`mdbook init --theme --force` was supposed to not overwrite but did.
