---
id: starship
aliases: []
tags: []
---

```toml
## FIRST LINE/ROW: Info & Status
# First param ─┌
[username]
format = " [╭─$user]($style)@"
show_always = true
style_root = "bold red"
style_user = "bold red"

# Second param
[hostname]
disabled = false
format = "[$hostname]($style) in "
ssh_only = false
style = "bold dimmed red"
trim_at = "-"

# Third param
[directory]
style = "purple"
truncate_to_repo = true
truncation_length = 0
truncation_symbol = "repo: "

# Fourth param
[sudo]
disabled = false

# Before all the version info (python, nodejs, php, etc.)
[git_status]
ahead = "⇡${count}"
behind = "⇣${count}"
deleted = "x"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
style = "white"

# Last param in the first line/row
[cmd_duration]
disabled = false
format = "took [$duration]($style)"
min_time = 1

## SECOND LINE/ROW: Prompt
# Somethere at the beginning
[battery]
charging_symbol = ""
disabled = true
discharging_symbol = ""
full_symbol = ""

[[battery.display]] # "bold red" style when capacity is between 0% and 15%
disabled = false
style = "bold red"
threshold = 15

[[battery.display]] # "bold yellow" style when capacity is between 15% and 50%
disabled = true
style = "bold yellow"
threshold = 50

[[battery.display]] # "bold green" style when capacity is between 50% and 80%
disabled = true
style = "bold green"
threshold = 80

# Prompt: optional param 1
[time]
disabled = true
format = " 🕙 $time($style)\n"
style = "bright-white"
time_format = "%T"

# Prompt: param 2
[character]
error_symbol = " [×](bold red)"
success_symbol = " [╰─λ](bold red)"

# SYMBOLS
[status]
disabled = false
format = '[\[$symbol$status_common_meaning$status_signal_name$status_maybe_int\]]($style)'
map_symbol = true
pipestatus = true
symbol = "🔴"

[aws]
symbol = " "

[conda]
symbol = " "

[dart]
symbol = " "

[docker_context]
symbol = " "

[elixir]
symbol = " "

[elm]
symbol = " "

[git_branch]
symbol = " "

[golang]
symbol = " "

[hg_branch]
symbol = " "

[java]
symbol = " "

[julia]
symbol = " "

[nim]
symbol = " "

[nix_shell]
symbol = " "

[nodejs]
symbol = " "

[package]
symbol = " "

[perl]
symbol = " "

[php]
symbol = " "

[python]
symbol = " "

[ruby]
symbol = " "

[rust]
symbol = " "

[swift]
symbol = "ﯣ "

```
