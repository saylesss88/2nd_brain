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

- [Google Fonts Iosevka Charon Mono](https://fonts.google.com/specimen/Iosevka+Charon+Mono)

```bash
mkdir -p ~/.local/share/fonts/iosevka-charon-mono
cd ~/Downloads && mkdir iosevka-charon-mono
unzip Iosevka_Charon_Mono.zip -d iosevka-charon-mono/
cd iosevka-charon-mono
cp * ~/.local/share/fonts/iosevka-charon-mono
fc cache -fv
```

## Waybar style.css

```css
* {
  border: none;
  font-family: "JetBrainsMono Nerd Font", "Font Awesome 6 Free", sans-serif;
  font-size: 14px;
}
```
