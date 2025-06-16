---
id: Record and playback commands
aliases:
  - Record and playback commands
tags: []
---

# Record and playback commands

There are three steps to command recording:

1. The "q{register}" command starts recording keystrokes into the register
   named {register}.

2. Type your commands

3. To finish recording, press q (without any extra character)

You can now execute the macro by typing the command "@{register}"

Take a look at how to use these commands in practice. You have a list of
filenames that look like this:

stdio.h
fcntl.h
unistd.h
stdlib.h

And what you want is the following:

#include "stdio.h"
#include "fcntl.h"
#include "unistd.h"
#include "stdlib.h"

You start by moving to the first character of the first line. Next you
execute the following commands:
qa Start recording a macro in register a.
^ Move to the beginning of the line.
i#include "<Esc> Insert the string #include " at the beginning
of the line.
$ Move to the end of the line.
a"<Esc> Append the character double quotation mark (")
to the end of the line.
j Go to the next line.
q Stop recording the macro.

Now that you have done the work once, you can repeat the change by typing the
command "@a" three times.
The "@a" command can be preceded by a count, which will cause the macro to
be executed that number of times. In this case you would type:

3@a
