---
id: Recording a macro
aliases:
  - Recording a macro
tags: []
---

# Recording a macro

Each register is identified by a letter `a` to `z`.

To enter a macro, type:

```vim
q<letter><commands>q
```

To execute the macro <number> times (once by default), type:

```vim
<number>@<letter>
``
So, the complete process looks like:
`qd` start recording to register `d`
`...` your complex series of commands
`q` stop recording
`@d` execute your macro
`@@` execute your macro again

## Example

Given some data like the following:

```shell
one': 'first example
two	second example
three	third example
four	fourth example
```

suppose you want to change the data to make a dictionary for a Python program
with the result:

```python
data = {
    'one': 'first example',
    'two': 'second example',
    'three': 'third example',
    'four': 'fourth example',
}
```

To do this, record a macro while changing the first line. Then, playback the
macro to change each other line. When finished, manually insert the initial
"data = {" line, and the final "}" line.

The following shows one way to record a suitable macro.

    Put the cursor on the first line.
    Type qd (the q starts recording; the d is the register where keys will be recorded).
    Type the following command to change the first sequence of whitespace to "': '":

        :s/\s\+/': ' (then press Enter)

    Type the following to insert four spaces followed by "'" at the start of the line:

        I    ' (then press Escape)

    Type the following to append "'," to the line:

        A', (then press Escape)

    Type the following to move the cursor to the start of the line, then down to the next line:

        0j (or press Enter)

    Type q to stop recording the macro.
