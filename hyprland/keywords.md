---
id: keywords
aliases: []
tags: []
---


Keywords are not variables, but "commands" for more advanced configuring.

## Executing

You can execute a shell script on startup of the compositor or every time the
config is reloaded.

  `exec-once = command` will execute only on launch
  `exec = command` will execute on each reload.

### Defining Variables

You can define your own custom variables using a dollar sign (`$`):

```hyper Lang
$VAR = value
```

for example:

```hyper Lang
$MyFavoriteGame = Among Us
```

Then you can reference them in the rest of the config. For example:

```hyper Lang
col.active_border = $MyColor
```

You are allowed to combine variables in-place with other strings, like this:

```hyper Lang
col.active_border = ff$MyRedValue1111
```

### Sourcing (multi-file)

Use the `source` keyword to source another file.

For example, in your `hyprland.conf` you can:

```hyper Lang
source = ~/.config/hypr/myColors.conf
```

And Hyprland will enter that file and parse it like a Hyprland config.
