# cli.rs explained

This cli.rs file is the "brain" of your interface. By using clap's derive
macros, you’ve turned a simple Rust struct into a powerful command-line parser
that handles validation, help menus, and type conversion automatically.

Here is the breakdown of how these specific parts work together.

---

1. The Backend Enum & ValueEnum

By deriving clap::ValueEnum, you are telling clap that this isn't just a
internal data type—it's a list of valid choices for the user.

- Case Insensitivity: By default, clap will allow the user to type
  `--backend sway` or `--backend Sway`.

- Validation: If a user tries to run your app with `--backend gnome`, clap will
  automatically print an error message listing the valid choices and exit before
  your main function even starts.

- Safety: In your `main.rs` match statement, you don't have to worry about a
  "default" or "unknown" case because the compiler knows Backend can only be
  Sway or Hyprpaper.

---

This cli.rs file is the "brain" of your interface. By using clap's derive
macros, you’ve turned a simple Rust struct into a powerful command-line parser
that handles validation, help menus, and type conversion automatically.

Here is the breakdown of how these specific parts work together.

1. The Backend Enum & ValueEnum By deriving clap::ValueEnum, you are telling
   clap that this isn't just a internal data type—it's a list of valid choices
   for the user.

Case Insensitivity: By default, clap will allow the user to type --backend sway
or --backend Sway.

Validation: If a user tries to run your app with --backend gnome, clap will
automatically print an error message listing the valid choices and exit before
your main function even starts.

Safety: In your main.rs match statement, you don't have to worry about a
"default" or "unknown" case because the compiler knows Backend can only be Sway
or Hyprpaper.

---

2. PathBuf: Smart Path Handling

You used `PathBuf` for `wallpaper_dir`. This is much better than using a String.

- Cross-Platform: It handles the differences between Unix `/` and Windows `\`
  paths.

- Validation: Because `clap` knows it's a `PathBuf`, it ensures the input is a
  valid string for a file path.

---

3. The Cli Struct: Attributes & Flags

This is where you define the "Shape" of your command line.

**The Metadata**

`#[clap(author, version, about)]` pulls information directly from your
`Cargo.toml`. When a user runs `your_app --version`, it will display the version
number you defined in your project settings.

| Field           | Macro Detail  | Effect                          |
| :-------------- | :------------ | :------------------------------ |
| `wallpaper_dir` | Positional    | my_app ./Pictures               |
| `time`          | `short, long` | `-t, --time`                    |
| `outputs`       | `Vec<String>` | can pass multiple times         |
| `backend`       | `value_enum`  | List the flag of `Backend` enum |

---

## How it looks to the user

`clap` automatically generates a polished `--help` screen:

```bash
A lightweight Wayland wallpaper daemon that randomizes backgrounds per-screen.

Usage: wallpaper-daemon [OPTIONS] <WALLPAPER_DIR>

Arguments:
  <WALLPAPER_DIR>  Directory containing wallpapers (supports jpg, png, bmp)

Options:
  -t, --time <TIME>        Time between wallpaper changes [default: 30m]
  -o, --outputs <OUTPUTS>  Output names to target
  -b, --backend <BACKEND>  Backend to use [default: sway] [possible values: sway, hyprpaper]
  -h, --help               Print help
  -V, --version            Print version
```
