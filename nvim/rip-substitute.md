---
id: rip-substitute
aliases:
  - rip-substitute
tags: []
---

# rip-substitute

Perform search and replace operations in the current buffer using a modern
interface and contemporary regex syntax.

Search and replace in the current buffer using ripgrep.
Uses common regex syntax — no more dealing with arcane vim regex.
Incremental preview of matched strings and replacements, live count of matches.
Popup window instead of command line. This entails:

    Syntax highlighting of the regex.
    Editing with vim motions.
    No more dealing with delimiters.

Sensible defaults: entire buffer (%), all matches in a line (/g), case-sensitive (/I).
Substitute only in a range, with visual emphasis of the range
History of previous substitutions.
Performant: In a file with 5000 lines and thousands of matches, still performs blazingly fast.™
Regex101 integration: Open the planned substitution in a pre-configured regex101 browser-tab for debugging.
Quality-of-Life features: automatic prefill of the escaped cursorword, adaptive popup window width, toggle --fixed-strings, …
Syntax comparison:

```vim
# all three are equivalent

# vim's :substitute
:% s/\(foo\)bar\(\.\)\@!/\1baz/gI

# vim's :substitute (very magic mode)
:% s/\v(foo)bar(\.)@!/\1baz/gI

# rip-substitute
(foo)bar(?!\.)
$1baz
```

## Installation

```lua
{
	"chrisgrieser/nvim-rip-substitute",
	cmd = "RipSubstitute",
	keys = {
		{
			"<leader>fs",
			function() require("rip-substitute").sub() end,
			mode = { "n", "x" },
			desc = " rip substitute",
		},
	},
},
```

### Default Settings

```lua
require("rip-substitute").setup {
	popupWin = {
		title = " rip-substitute",
		border = "single",
		matchCountHlGroup = "Keyword",
		noMatchHlGroup = "ErrorMsg",
		hideSearchReplaceLabels = false,
		---@type "top"|"bottom"
		position = "bottom",
	},
	prefill = {
		---@type "cursorWord"| false
		normal = "cursorWord",
		---@type "selectionFirstLine"| false does not work with ex-command (see README).
		visual = "selectionFirstLine",
		startInReplaceLineIfPrefill = false,
	},
	keymaps = { -- normal & visual mode, if not stated otherwise
		abort = "q",
		confirm = "<CR>",
		insertModeConfirm = "<C-CR>",
		prevSubst = "<Up>",
		nextSubst = "<Down>",
		toggleFixedStrings = "<C-f>", -- ripgrep's `--fixed-strings`
		toggleIgnoreCase = "<C-c>", -- ripgrep's `--ignore-case`
		openAtRegex101 = "R",
	},
	incrementalPreview = {
		matchHlGroup = "IncSearch",
		rangeBackdrop = {
			enabled = true,
			blend = 50, -- between 0 and 100
		},
	},
	regexOptions = {
		startWithFixedStringsOn = false,
		startWithIgnoreCase = false,
		-- pcre2 enables lookarounds and backreferences, but performs slower
		pcre2 = true,
		-- disable if you use named capture groups (see README for details)
		autoBraceSimpleCaptureGroups = true,
	},
	editingBehavior = {
		-- When typing `()` in the `search` line, automatically adds `$n` to the
		-- `replace` line.
		autoCaptureGroups = false,
	},
	notificationOnSuccess = true,
}
```

#### Usage

```lua
vim.keymap.set(
	{ "n", "x" },
	"<leader>fs",
	function() require("rip-substitute").sub() end,
	{ desc = " rip substitute" }
)
```

- Normal mode: prefills the escaped word under the cursor.
- Visual mode: prefills the escaped selection.
- Visual line mode: replacements are only applied to the selected lines (= the
  selection is used as a range)

**Ex-Command**

```vim
" Substitute in entire file. Prefills the *escaped* word under the cursor.
:RipSubstitute

" Substitute in line range of the visual selection.
:'<,'>RipSubstitute

" Substitute in given range (in this case: current line to end of file).
:.,$ RipSubstitute
```

##### Advanced

**Remember prefill**
The function `requirt("rip-substitute).rememberCursorWord()` can be used to save
the word under the cursor for the next time `rip-substitute` is invoked.

- One use case for this is to set a prefill for when you intend to run
  substitute with a range, since calling `rip-substitute` in visual line is not
  able to pick up a prefill.

**Filetype**
