---
id: Nvim SHADA (Shared Data)file and Sessions
aliases:
  - SHADA (Shared Data) file and Sessions
tags: []
---

# SHADA (Shared Data) file and Sessions

Ever stop your vim and get really depressed that you just lost your buffers?
Cause I've done it twice today. The relevant details about sessions:

```vim
The following command creates a session file:

	:mksession vimbook.vim

Later if you want to restore this session, you can use this command:

	:source vimbook.vim

If you want to start Vim and restore a specific session, you can use the
following command:

	vim -S vimbook.vim

This tells Vim to read a specific file on startup.  The 'S' stands for
session (actually, you can source any Vim script with -S, thus it might as
well stand for "source").
```

Your `sessionoptions` needs to include `buffers`. The SHADA file is like adding
steroids. You can keep marks, register contents, and command line history.
