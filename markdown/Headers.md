---
id: Headers
aliases: []
tags: []
---


Markdown offers two styles of headers: `Setext` and `atx`
- Setext-style headers for `<h1>` and `<h2>` are created by "underlining" with
  equal signs (`=`) and hyphens (`-`).

- Atx-style headers are created with 1-6 hash marks `#` at the beginning of the
  line.

- Blockquotes are indicated using email-style '`>`' angle brackets.

```md
A First Level Header
====================

A Second Level Header
---------------------

Now is the time for all good men to come to
the aid of their country. This is just a
regular paragraph.

The quick brown fox jumped over the lazy
dog's back.

### Header 3

> This is a blockquote.
> 
> This is the second paragraph in the blockquote.
>
> ## This is an H2 in a blockquote
```

Output:

```md
<h1>A First Level Header</h1>

<h2>A Second Level Header</h2>

<p>Now is the time for all good men to come to
the aid of their country. This is just a
regular paragraph.</p>

<p>The quick brown fox jumped over the lazy
dog's back.</p>

<h3>Header 3</h3>

<blockquote>
    <p>This is a blockquote.</p>

    <p>This is the second paragraph in the blockquote.</p>

    <h2>This is an H2 in a blockquote</h2>
</blockquote>
```

[[Lists.md]]
