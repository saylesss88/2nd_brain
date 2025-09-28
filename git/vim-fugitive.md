# Fugitive.vim - a complement to command line git

Using the :Git command, you can run any arbitrary git command from inside Vim. I prefer to switch to the shell for anything that generates a log of output, such as git log for example. But commands that generate little or no output are fair game for running from inside Vim (:Git checkout -b experimental for example).

At Vim’s command line, the % symbol has a special meaning: it expands to the full path of the current file. You can use this to run any git command that expects a filepath as an argument, making the command act on the current file. But fugitive also provides a few convenience methods, some of which are summarized in this table:
git fugitive action
:Git add % :Gwrite Stage the current file to the index
:Git checkout % :Gread Revert current file to last checked in version
:Git rm % :Gremove Delete the current file and the corresponding Vim buffer
:Git mv % :Gmove Rename the current file and the corresponding Vim buffer

The :Gcommit command opens up a commit window in a split window. One advantage to using this, rather than running git commit in the shell, is that you can use Vim’s keyword autocompletion when composing your commit message.

The :Gblame command opens a vertically split window containing annotations for each line of the file: the last commit reference, with author and timestamp. The split windows are bound, so that when you scroll one, the other window will follow.
Further Reading

    :help cmdline-special
    :help :_%
    ctlr-n/ctrl-p keyword autocompletion
    :help 'complete'
    :help :Git
    :help :Gwrite
    :help :Gread
    :help :Gremove
    :help :Gmove
    :help :Gcommit
    :help :Gblame
