---
id: zsh_completion_system
aliases:
  - zsh
  - zsh completion system
tags: []
---

# zsh completion system

To initialize the completion for the current Zsh session, you'll need to call
the function `compinit`. Add this to your `zshrc`:

```zsh
autoload -U compinit; compinit
```

What does this mean?

The `autoload` command loads a file containing shell commands.

- To find this file, Zsh will look in the directories of the Zsh file search
  path, defined in the variable `$fpath`, and search a file called `compinit`.

- When compinit is found, its content will be loaded as a function. The function
  name will be the name of the file. You can then call this function like any other
  shell function.

> [!TIP] What about the semi-colon ;? It’s just a handy way to separate commands.
> It’s the same as calling compinit on a new line.

Why using `autoload`, and not sourcing the file by doing source `~/path/of/compinit`?

- It avoids name conflicts if you have an executable with the same name.

- It doesn't expand aliases thanks to the `-U` option.

- It will load the function only when it's needed (lazy-loading). It comes in
  handy to speed up Zsh startup time.

Then, let's add the following:

```zsh
_comp_options+=(globdots) # With hidden files
source ~/.config/zsh/completion.zsh
```
