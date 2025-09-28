## Dynamic Filtering

_Static filtering_ refers to the filters which come from filter lists, i.e.,
EasyList, EasyPrivacy, URLhaus Blocklist, etc. _Dynamic filtering_ are those
filtering rules which have an air of firewall rules.

### Dynamic filtering rules override static filtering

This means a block dynamic rule will override any existing allow static filters.
This means you can block with 100% certainty using dynamic filtering rules.
Similarly, an allow dynamic filtering rule will override any existing block
static filters, i.e. you can allow with 100% certainty with dynamic filtering
(useful to un-break sites broken by some static filters).

- [uBlock Dynamic-filtering:-quick-guide](https://github.com/gorhill/uBlock/wiki/Dynamic-filtering:-quick-guide)

### The Columns

**First Column**: what is to be dynamically filtered.

**Second Column**: global dynamic filtering rules, i.e., whatever rule appears
in this column applies everywhere, on all sites.

**Third Column**: local dynamic filtering rules, i.e., whatever rule appears in
this column applies to the current site only.

The cells in the third column gives an overview of how many requests were
blocked/allowed:

    - or + = between 1-9 network requests were blocked or allowed, respectively
    -- or ++ = between 10-99 network requests were blocked or allowed, respectively
    --- or +++ = 100 or more network requests were blocked or allowed, respectively
    blank cell = no network requests occurred for the specific hostname

So there are global dynamic filtering rules, and local dynamic filtering rules.

By default, there are no dynamic filtering rules at install time, so nothing is
blocked by default by the dynamic filtering engine. You will have to create your
own rules, according to your own prerogatives.

---

#### Block rules

Sensible security and privacy-wise is blocking all 3rd-party frames by default
everywhere.

If there is something embedded in the page that you want to see, such as a
YouTube video. You have a few options:

- Set _Noop rules_, by clicking on the 3rd column of `3rd-party frames`. The
  cell will be dark gray, while a cell with no rule is light gray. Hence, gray
  means that no dynamic filtering will be applied to a cell.
