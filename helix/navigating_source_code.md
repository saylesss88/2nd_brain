# Helix Navigation

**Go-to (the most important)**

- `gd` — go to definition. Stand on any type or function, jump to where it's defined
- `gr` — go to references. See everywhere something is used
- `gy` — go to type definition
- `gi` — go to implementation
- `C-o` — jump back (like a back button through your jump history)


**File navigation**

- `Space-f` — file picker
- `Space-b` — buffer picker (open files)
- `Space-s` — symbol picker for current file (great for jumping to a specific fn/struct)
- `Space-S` — workspace symbol picker (search symbols across the whole project)

- `Space-S` is huge for reading source — type Controller and jump straight to it anywhere in the codebase.

**Search**

- `/` — search in file
- `Space-/` — grep across workspace (ripgrep). Type a function name and find every occurrence in the project


**Diagnostics/hover**

- `Space-k` — hover docs (shows the rustdoc for whatever your cursor is on)
- `]d` / `[d` — next/previous diagnostic


**Practical workflow for bat**

1. `Space-S` → type `Controller` → jump to the struct definition
2. `gr` on `Controller::run` → see every callsite
3. `C-o` → jump back where you were
