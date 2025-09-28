---
id: Events
aliases:
  - Events
tags: []
---

# Events

- You can specify a comma-separated list of event names. No white space can be
  used in the list. The command applies to all the events in the list.

For READING FILES there are four kinds of events possible:
BufNewFile starting to edit a non-existent file
BufReadPre BufReadPost starting to edit an existing file
FilterReadPre FilterReadPost read the temp file with filter output
FileReadPre FileReadPost any other file read

- Vim uses only one of these four kinds when reading a file. The "Pre" and "Post"
  events are both triggered, before and after the reading process.

**BufAdd** : After adding a new buffer or existing unlisted buffer to the buffer list
(except during startup), or renaming a listed buffer.

BufDelete Before deleting a buffer from the buffer list.
The BufUnload may be called first (if the
buffer was loaded).
Also used just before a buffer in the buffer
list is renamed.
NOTE: Current buffer "%" is not the target
buffer "<afile>", "<abuf>". |<buffer=abuf>|
Do not change to another buffer.
_BufEnter_
BufEnter After entering (visiting, switching-to) a new
or existing buffer. Useful for setting
filetype options. Compare |BufNew| which
does not trigger for existing buffers.
After |BufAdd|.
After |BufReadPost|.
_BufFilePost_
BufFilePost After changing the name of the current buffer
with the ":file" or ":saveas" command.
_BufFilePre_
BufFilePre Before changing the name of the current buffer
with the ":file" or ":saveas" command.
_BufHidden_
BufHidden Before a buffer becomes hidden: when there are
no longer windows that show the buffer, but
the buffer is not unloaded or deleted.

    			Not used for ":qa" or ":q" when exiting Vim.
    			NOTE: Current buffer "%" is not the target
    			buffer "<afile>", "<abuf>". |<buffer=abuf>|
    						*BufLeave*

BufLeave Before leaving to another buffer. Also when
leaving or closing the current window and the
new current window is not for the same buffer.

    			Not used for ":qa" or ":q" when exiting Vim.
    						*BufModifiedSet*

BufModifiedSet After the `'modified'` value of a buffer has
been changed.
_BufNew_
BufNew After creating a new buffer (except during
startup, see |VimEnter|) or renaming an
existing buffer. Unlike |BufEnter|, visiting
(switching to) an existing buffer will not
trigger this again.
NOTE: Current buffer "%" is not the target
buffer "<afile>", "<abuf>". |<buffer=abuf>|
See also |BufAdd|, |BufNewFile|.
_BufNewFile_
BufNewFile When starting to edit a file that doesn't
exist. Can be used to read in a skeleton
file.
_BufRead_ _BufReadPost_
BufRead or BufReadPost When starting to edit a new buffer, after
reading the file into the buffer, before
processing modelines. See |BufWinEnter| to do
something after processing modelines.
Also triggered: - when writing an unnamed buffer in a way that
the buffer gets a name - after successfully recovering a file - for the "filetypedetect" group when
executing ":filetype detect"
Not triggered: - for the `:read file` command - when the file doesn't exist
_BufReadCmd_
BufReadCmd Before starting to edit a new buffer. Should
read the file into the buffer. |Cmd-event|
_BufReadPre_ _E200_ _E201_
BufReadPre When starting to edit a new buffer, before
reading the file into the buffer. Not used
if the file doesn't exist.
_BufUnload_
BufUnload Before unloading a buffer, when the text in
the buffer is going to be freed.
After BufWritePost.
Before BufDelete.
Triggers for all loaded buffers when Vim is
going to exit.
NOTE: Current buffer "%" is not the target
buffer "<afile>", "<abuf>". |<buffer=abuf>|
Do not switch buffers or windows!
Not triggered when exiting and v:dying is 2 or
more.
_BufWinEnter_
BufWinEnter After a buffer is displayed in a window. This
may be when the buffer is loaded (after
processing modelines) or when a hidden buffer
is displayed (and is no longer hidden).

    			Not triggered for |:split| without arguments,
    			since the buffer does not change, or :split
    			with a file already open in a window.
    			Triggered for ":split" with the name of the
    			current buffer, since it reloads that buffer.
    						*BufWinLeave*

BufWinLeave Before a buffer is removed from a window.
Not when it's still visible in another window.
Also triggered when exiting.
Before BufUnload, BufHidden.
NOTE: Current buffer "%" is not the target
buffer "<afile>", "<abuf>". |<buffer=abuf>|
Not triggered when exiting and v:dying is 2 or
more.
_BufWipeout_
BufWipeout Before completely deleting a buffer. The
BufUnload and BufDelete events may be called
first (if the buffer was loaded and was in the
buffer list). Also used just before a buffer
is renamed (also when it's not in the buffer
list).
NOTE: Current buffer "%" is not the target
buffer "<afile>", "<abuf>". |<buffer=abuf>|
Do not change to another buffer.
_BufWrite_ _BufWritePre_
BufWrite or BufWritePre Before writing the whole buffer to a file.
_BufWriteCmd_
BufWriteCmd Before writing the whole buffer to a file.
Should do the writing of the file and reset
'modified' if successful, unless '+' is in
'cpo' and writing to another file |cpo-+|.
The buffer contents should not be changed.
When the command resets 'modified' the undo
information is adjusted to mark older undo
states as 'modified', like |:write| does.
|Cmd-event|
_BufWritePost_
BufWritePost After writing the whole buffer to a file
(should undo the commands for BufWritePre).
_ChanInfo_
ChanInfo State of channel changed, for instance the
client of a RPC channel described itself.
Sets these |v:event| keys:
info
See |nvim_get_chan_info()| for the format of
the info Dictionary.
_ChanOpen_
ChanOpen Just after a channel was opened.
Sets these |v:event| keys:
info
See |nvim_get_chan_info()| for the format of
the info Dictionary.
_CmdUndefined_
CmdUndefined When a user command is used but it isn't
defined. Useful for defining a command only
when it's used. The pattern is matched
against the command name. Both <amatch> and
<afile> expand to the command name.
NOTE: Autocompletion won't work until the
command is defined. An alternative is to
always define the user command and have it
invoke an autoloaded function. See |autoload|.
_CmdlineChanged_
CmdlineChanged After a change was made to the text inside
command line. Be careful not to mess up the
command line, it may cause Vim to lock up.
<afile> expands to the |cmdline-char|.
_CmdlineEnter_
CmdlineEnter After entering the command-line (including
non-interactive use of ":" in a mapping: use
|<Cmd>| instead to avoid this).
The pattern is matched against |cmdline-char|.
<afile> expands to the |cmdline-char|.
Sets these |v:event| keys:
cmdlevel
cmdtype
_CmdlineLeave_
CmdlineLeave Before leaving the command-line (including
non-interactive use of ":" in a mapping: use
|<Cmd>| instead to avoid this).
<afile> expands to the |cmdline-char|.
Sets these |v:event| keys:
abort (mutable)
cmdlevel
cmdtype
Note: `abort` can only be changed from false
to true: cannot execute an already aborted
cmdline by changing it to false.
_CmdwinEnter_
CmdwinEnter After entering the command-line window.
Useful for setting options specifically for
this special type of window.
<afile> expands to a single character,
indicating the type of command-line.
|cmdwin-char|
_CmdwinLeave_
CmdwinLeave Before leaving the command-line window.
Useful to clean up any global setting done
with CmdwinEnter.
<afile> expands to a single character,
indicating the type of command-line.
|cmdwin-char|
_ColorScheme_
ColorScheme After loading a color scheme. |:colorscheme|
Not triggered if the color scheme is not
found.
The pattern is matched against the
colorscheme name. <afile> can be used for the
name of the actual file where this option was
set, and <amatch> for the new colorscheme
name.

    						*ColorSchemePre*

ColorSchemePre Before loading a color scheme. |:colorscheme|
Useful to setup removing things added by a
color scheme, before another one is loaded.

CompleteChanged _CompleteChanged_
After each time the Insert mode completion
menu changed. Not fired on popup menu hide,
use |CompleteDonePre| or |CompleteDone| for
that.

    			Sets these |v:event| keys:
    			    completed_item	See |complete-items|.
    			    height		nr of items visible
    			    width		screen cells
    			    row			top screen row
    			    col			leftmost screen column
    			    size		total nr of items
    			    scrollbar		TRUE if visible

    			Non-recursive (event cannot trigger itself).
    			Cannot change the text. |textlock|

    			The size and position of the popup are also
    			available by calling |pum_getpos()|.

    						*CompleteDonePre*

CompleteDonePre After Insert mode completion is done. Either
when something was completed or abandoning
completion. |ins-completion|
|complete_info()| can be used, the info is
cleared after triggering CompleteDonePre.
The |v:completed_item| variable contains
information about the completed item.

    						*CompleteDone*

CompleteDone After Insert mode completion is done. Either
when something was completed or abandoning
completion. |ins-completion|
|complete_info()| cannot be used, the info is
cleared before triggering CompleteDone. Use
CompleteDonePre if you need it.
|v:completed_item| gives the completed item.

    			Sets these |v:event| keys:
    			    reason	Reason for completion being
    					done. Can be one of:
    					- "accept": completion was
    					  accepted using |complete_CTRL-Y|.
    					- "cancel": completion was cancelled
    					  using |complete_CTRL-E|, pressing
    					  a non-keyword character, or
    					  triggering a new completion.

    						*CursorHold*

CursorHold When the user doesn't press a key for the time
specified with 'updatetime'. Not triggered
until the user has pressed a key (i.e. doesn't
fire every 'updatetime' ms if you leave Vim to
make some coffee. :) See |CursorHold-example|
for previewing tags.
This event is only triggered in Normal mode.
It is not triggered when waiting for a command
argument to be typed, or a movement after an
operator.
While recording the CursorHold event is not
triggered. |q|
_<CursorHold>_
Internally the autocommand is triggered by the
<CursorHold> key. In an expression mapping
|getchar()| may see this character.

    			Note: Interactive commands cannot be used for
    			this event.  There is no hit-enter prompt,
    			the screen is updated directly (when needed).
    			Note: In the future there will probably be
    			another option to set the time.
    			Hint: to force an update of the status lines
    			use: >
    				:let &ro = &ro

<
_CursorHoldI_
CursorHoldI Like CursorHold, but in Insert mode. Not
triggered when waiting for another key, e.g.
after CTRL-V, and not in CTRL-X mode
|insert_expand|.

    						*CursorMoved*

CursorMoved After the cursor was moved in Normal or Visual
mode or to another window. Also when the text
of the cursor line has been changed, e.g. with
"x", "rx" or "p".
Not always triggered when there is typeahead,
while executing commands in a script file, or
when an operator is pending.
For an example se

[[Signal_events.md]]
