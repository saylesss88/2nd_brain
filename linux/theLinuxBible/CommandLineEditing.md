---
id: CommandLineEditing
aliases:
  - CommandLineEditing
tags: []
---

# CommandLineEditing

If you type something wrong on a command line, the bash shell ensures that you
don't have to delete the entire line and start over.

- By default, the bash shell uses command-line editing that is based on the
  emact text editor. 

- To switch to the better vi use `set -o vi` in your .bashrc

Say you entered `ls /usr/bin | sort -f | less` and wanted to change `/usr/bin` to `/bin` You could:

1. Press the up key. To display the most recent command from your shell history.

2. Press Ctrl+A. To move the cursor to the start of the line.

3. Press Ctrl+F or right arrow. to position the cursor under the first slash (/).

4. Press Ctrl+D. Type this command four times to delete `/usr` from the line.

5. Press Enter. Execute the line.

Keystrokes for Navigating Command Lines
----------------------------------------------------------------
Keystroke     Full Name            Meaning
Ctrl+F      Character forward      Go forward one character.

Ctrl+B      Character backward     Go backward one character.

Alt+F       Word forward           Go forward one word.

Alt+B       Word backward          Go backward one word.

Ctrl+A      Beginning of line      Go to the beginning of the current line.

Ctrl+E      End of line            Go to the end of the line.

Ctrl+L      Clear screen           Clear screen and leave line at the top of the screen.

The keystrokes in Table 3.2 can be used to edit command lines.

 Keystrokes for Editing Command Lines
------------------------------------------------------------------------------
Keystroke        Full Name
**Ctrl+D**           Delete current

**Backspace **       Delete previous

**Ctrl+T**           Transpose character

**Alt+T**            Transpose words

**Alt+U**            Uppercase word

**Alt+L**            Lowercase word

**Alt+C**            Capitalize word

**Ctrl+V**           Insert special character

**Keystrokes for Cutting and Pasting Text from within Command Lines**
------------------------------------------------------------------------------
**Keystroke**               **Full Name**                 **Meaning**
**Ctrl+K**                  Cut end of line       Cut to end of line

**Ctrl+U**              Cut beginning of line Cut to beginning of line

**Ctrl+W**               Cut prev. word              Cut word behind cursor

**Alt+D**                Cut next word               Cut word following cursor

**Ctrl+Y**               Paste                       Paste most recent cut

**Alt+Y**           Paste earlier text       Rotate back to previous cut

**Ctrl+C**          Delete whole line        Delete entire line

### Command-Line Completion

Here are some values you can type partially from a bash shell:

- **Command, alias, or function** 

- **Variable**

- **Username**

- **Hostname**

> [!TIP] To add hostnames from an additional file, you can set the `HOSTFILE`
> variable to the name of that file. The file must be in the same format as
> `/etc/hosts`

[[Command-lineRecall.md]]
