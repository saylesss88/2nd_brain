## Overriding the palette (Dracula, etc.)

This is still a WIP, currently the overrides only apply to certain elements.
Dracula is just an example, the idea is to allow overriding to whatever you
prefer, just change the values in the provided `dracula.css` to what you like.

![screenshot1](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/swappy-20251130-142446.cleaned.png)

<details>
<summary> ✔️ Click to Expand override Example </summary>

You can still override the color palette while keeping the Kanagawa layout.

Create `your-book/src/assets/dracula.css`:

```css
/* src/assets/dracula.css */

html.navy,
body.navy,
.navy {
  /* Core page colors */
  --bg: #282a36;
  --bg-alt: #44475a;
  --fg: #f8f8f2;
  --fg-light: #cfcfd9;

  /* Waves / accent palette */
  --wave-1: #282a36;
  --wave-2: #343746;
  --wave-3: #44475a;

  --accent: #bd93f9;
  --red: #ff5555;
  --blue: #8be9fd;

  /* mdBook-specific navy vars */
  --sidebar-bg: #282a36;
  --sidebar-fg: #f8f8f2;
  --sidebar-non-existant: #6272a4;
  --sidebar-active: #bd93f9;
  --sidebar-spacer: #44475a;

  --links: #bd93f9;

  --quote-bg: #343746;
  --quote-border: #44475a;

  --table-header-bg: #44475a;
  --table-alternate-bg: #343746;

  /* tweak others as you like */
}
```

When you run `mdbook build` this file will be placed in
`your-book/book/assets/dracula.css`.

And in `book.toml`:

```toml
[preprocessor.kanagawa-theme]
renderers = ["html"]
before = ["content-loader", "content-collections"]

landing_title = "My mdBook"
landing_subtitle = "Notes, posts, and more"

# These strings can be changed to whatever you prefer
header_latest = "Latest posts"
header_notes = "Recent notes"
header_tags = "Popular tags"

css_import = "/assets/dracula.css"
disable_builtin_css = false
```

With this setup:

- mdBook still builds the book as usual, and `mdbook-kanagawa-theme` generates
  `theme/css/chrome.css` with the Kanagawa layout and variable hooks.​

- The preprocessor adds an `@import "/assets/dracula.css";` (from `css_import`)
  at the top of that generated `chrome.css`, so your Dracula file runs after the
  built‑in variable defaults.​

- Because `dracula.css` redefines the same CSS custom properties used by the
  Kanagawa theme (`--bg`, `--fg`, `--accent`, sidebar colors, etc.), the page
  keeps the Kanagawa layout and landing page, but all colors for the `navy`
  theme class come from your Dracula palette instead. Kanagawa provides the
  layout and default palette.

- The theme is only overridden currently for `Auto`, and `Navy` from the
  dropdown.

</details>

---
