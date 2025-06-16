---
id: zathura
aliases: []
tags: []
---

## Usage

zathura uses vi-like key map by default.

Commands may be entered directly into zathura by pressing `:`, just like vim.

aliases:

`alias zathura="zathura --fork"`
`alias -s pdf=zathura --fork`

MOUSE AND KEY BINDINGS
General

          J, PgDn
                 Go to the next page

          K, PgUp
                 Go to the previous page

          h, k, j, l
                 Scroll to the left, down, up or right direction

          Left, Down, Up, Right
                 Scroll to the left, down, up or right direction

          ^t, ^d, ^u, ^y
                 Scroll a half page left, down, up or right

          t, ^f, ^b, space, <S-space>, y
                 Scroll a full page left, down, up or right

          gg, G, nG
                 Goto to the first, the last or to the nth page

          P      Snaps to the current page

          H, L   Goto top or bottom of the current page

          ^o, ^i Move backward and forward through the jump list

           ^j, ^k Bisect forward and backward between the last two jump points

          ^c, Escape
                 Abort

          a, s   Adjust window in best-fit or width mode

          /, ?   Search for text

          n, N   Search for the next or previous result

          o, O   Open document

          f      Follow links

          F      Display link target

          c      Copy link target into the clipboard

          :      Enter command

          r      Rotate by 90 degrees

          ^r     Recolor (grayscale and invert colors)

          R      Reload document

          Tab    Show index and switch to Index mode

          d      Toggle dual page view

           D      Cycle opening column in dual page view

          F5     Switch to presentation mode

          F11    Switch to fullscreen mode

          ^m     Toggle inputbar

          ^n     Toggle statusbar

          +, -, =
                 Zoom in, out or to the original size

          zI, zO, z0
                 Zoom in, out or to the original size

          n=     Zoom to size n

          mX     Set a quickmark to a letter or number X

          'X     Goto quickmark saved at letter or number X

          q      Quit

       Fullscreen mode

          J, K   Go to the next or previous page

          space, <S-space>, <BackSpace>
                 Scroll a full page down or up

           gg, G, nG
                 Goto to the first, the last or to the nth page

          ^c, Escape
                 Abort

          F11    Switch to normal mode

          +, -, =
                 Zoom in, out or to the original size

          zI, zO, z0
                 Zoom in, out or to the original size

          n=     Zoom to size n

          q      Quit

       Presentation mode

          space, <S-space>, <BackSpace>
                 Scroll a full page down or up

          ^c, Escape
                 Abort

          F5     Switch to normal mode

          q      Quit

            Index mode

          k, j   Move to upper or lower entry

          l      Expand entry

          L      Expand all entries

          h      Collapse entry

          H      Collapse all entries

          space, Return
                 Select and open entry

       Mouse bindings

          Scroll Scroll up or down

          ^Scroll
                 Zoom in or out

          Drag Button2 (middle button drag)
                 Pan the document

          Button1 (left click)
                 Follow link

          Drag Button1
                 Select text

           Button3 (right click)
                 Open popup menu to copy/save image (activates for images recognized by export command)

COMMANDS
bmark Save a bookmark.

       bdelete
              Delete a bookmark.

       blist  List bookmarks.

       bjump  Jump to given bookmark.

       jumplist
              Show recent jumps in jumplist (by default last 5). Optional argument specifies number of entries to show. Negative value "-N" shows all except the first  "N"
              entries.

       mark   Set a quickmark.

       delmarks
              Delete a quickmark. Abbreviation: delm.

       close  Close document.

        quit   Quit zathura. Abbreviation: q.

       exec   Execute  an external command. $FILE expands to the current document path, $PAGE to the current page number, and $DBUS to the bus name of the D-Bus interface.
              Alias: ! (space is still needed after).

       info   Show document information.

       open   Open a document. Abbreviation: o.

       offset Set page offset.

       print  Print document.

       write(!)
              Save document (and force overwriting). Alias: save(!).

       export Export attachments. First argument specifies the attachment identifier (use completion with Tab), second argument gives the target filename (relative to cur‐
              rent working directory).

       dump   Write values, descriptions, etc. of all current settings to a file.

       source Source a configuration file. It is possible to change the config directory by passing an argument.

       hlsearch
              Highlight current search results.

        nohlsearch
              Remove highlights of current search results. Abbreviation: nohl.

       version
              Show version information.

CONFIGURATION
The default appearance and behaviour of zathura can be overwritten by modifying the zathurarc file (default path: ~/.config/zathura/zathurarc). For a detailed de‐
scription please consult zathurarc(5).

SYNCTEX SUPPORT
Both synctex forward and backwards synchronization are supported by zathura, To enable synctex forward synchronization, please look at the --synctex-forward and
--synctex-editor options. zathura will also emit a signal via the D-Bus interface. To support synctex backwards synchronization, zathura provides a D-Bus interface
that can be called by the editor. For convince zathura also knows how to parse the output of the synctex view command. It is enough to pass the arguments to synctex
view's -i option to zathura via --synctex-forward and zathura will pass the information to the correct instance.

       For gvim forward and backwards synchronization support can be set up as follows: First add the following to the vim configuration:

          function! Synctex()
            execute "silent !zathura --synctex-forward " . line('.') . ":" . col('.') Manual page zathura(1) line 262 (press h for help or q to quit)
