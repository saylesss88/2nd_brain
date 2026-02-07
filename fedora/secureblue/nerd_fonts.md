# Nerd Fonts

```bash
mkdir -p ~/.local/share/fonts
```

```bash
cd Downloads
wget https://github.com/ryanoasis/nerd-fonts/releases/latest/download/JetBrainsMono.zip
unzip JetBrainsMono.zip -d ~/.local/share/fonts/JetBrainsMono
fc-cache -fv
```

## Waybar style.css

```css
* {
  border: none;
  font-family: "JetBrainsMono Nerd Font", "Font Awesome 6 Free", sans-serif;
  font-size: 14px;
}
```
